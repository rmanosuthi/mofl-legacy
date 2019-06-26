use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver};
use std::time::Duration;
use std::thread;
use std::thread::{JoinHandle, ThreadId};

use rand::Rng;

#[derive(Clone)]
pub enum WorkerReply {
    WorkerStarted,
    WorkerStopped,
    WorkerBusy(ThreadId),
    WorkerIdle(ThreadId),

    DownloadProgress(u64),
    DownloadFinished(String, PathBuf),
    DownloadFailed(String, Option<PathBuf>),
    DummyReply(String),
    DummyIntensiveTaskReply(u64)
}

#[derive(Clone)]
pub struct WorkerOrder {
    pub order: WorkerSend,
    pub result: glib::Sender<WorkerReply>
}

#[derive(Clone)]
pub enum WorkerSend {
    StopWorker,
    Park,
    
    StartDownload(String),
    PauseDownload(String),
    StopDownload(String),
    ImportMo2(PathBuf),
    DummySend(String),
    DummyIntensiveTask(u64)
}

#[derive(Clone)]
pub enum WorkerOccupation {
    Downloader,
    Importer,
    ModLoader,
    None
}

#[derive(Clone)]
pub enum SchedulerEvent {
    WorkerIsFree(ThreadId),
    WorkerIsBusy(ThreadId),
    AddTask(WorkerOrder)
}

pub struct Worker {
    handle: JoinHandle<()>,
    occupation: Arc<RwLock<WorkerOccupation>>,
    sender_to_worker: Sender<WorkerOrder>,
    busy: bool
}

impl Worker {
    pub fn new(occupation: WorkerOccupation, reply: glib::Sender<WorkerReply>) -> Worker {
        let (worker_sender_workerorder, worker_thread_receiver_workerorder) = channel::<WorkerOrder>();
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            debug!("Worker started");
            let thread_id = thread::current().id();
            loop {
                if let Ok(order) = worker_thread_receiver_workerorder.recv() {
                    let result = order.result;
                    match order.order {
                    WorkerSend::Park => thread::park(),
                    WorkerSend::DummyIntensiveTask(num) => {
                        debug!("{:?} <- Task with input {}", thread_id.clone(), num);
                        thread::sleep(Duration::from_millis(rng.gen_range(100, 900)));
                        reply.send(WorkerReply::WorkerBusy(thread_id.clone()));
                        reply.send(WorkerReply::DummyIntensiveTaskReply(num));
                        reply.send(WorkerReply::WorkerIdle(thread_id.clone()));
                        result.send(WorkerReply::DummyIntensiveTaskReply(num));
                    },
                    _ => ()
                    }
                }
            }
        });
        Worker {
            handle: handle,
            occupation: Arc::new(RwLock::new(occupation)),
            sender_to_worker: worker_sender_workerorder,
            busy: false
        }
    }

    pub fn get_thread_id(&self) -> ThreadId {
        return self.handle.thread().id();
    }

    pub fn pause(&self) {
        //self.sender_to_worker.send(WorkerSend::Park);
    }

    pub fn resume(&self) {
        self.handle.thread().unpark();
    }
}

pub struct WorkerWatcher {
    /*watcher_receiver: glib::Receiver<WorkerReply>,
    send_to_scheduler: Sender<SchedulerEvent>*/
}

impl WorkerWatcher {
    /*pub fn new() -> WorkerWatcher {
        WorkerWatcher {
            watcher_receiver: receiver,
            send_to_scheduler: send_to_scheduler
        }
    }*/
    pub fn watch(watcher_receiver: glib::Receiver<WorkerReply>, send_to_scheduler: Sender<SchedulerEvent>) {
        watcher_receiver.attach(None, move |reply| {
            match reply {
                WorkerReply::WorkerIdle(thread_id) => {
                    send_to_scheduler.send(SchedulerEvent::WorkerIsFree(thread_id));
                },
                WorkerReply::WorkerBusy(thread_id) => {
                    send_to_scheduler.send(SchedulerEvent::WorkerIsBusy(thread_id));
                },
                WorkerReply::DummyIntensiveTaskReply(num) => {
                    debug!("Received result {}", num);
                }
                _ => ()
            }
            glib::Continue(true)
        });
    }
}

pub struct WorkerManager {
    //scheduler: WorkerScheduler
    //watcher: WorkerWatcher
    send_to_scheduler: Sender<SchedulerEvent>
}

impl WorkerManager {
    pub fn new(threads: u64) -> WorkerManager {
        
        let (worker_sender_workerreply, watcher_receiver_workerreply) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let (send_to_scheduler, scheduler_receiver_tasks) = channel::<SchedulerEvent>();

        /*let watcher = WorkerWatcher::new(watcher_receiver_workerreply, send_to_scheduler.clone());
        watcher.watch();*/
        WorkerWatcher::watch(watcher_receiver_workerreply, send_to_scheduler.clone());

        let scheduler = WorkerScheduler::new(threads, worker_sender_workerreply, scheduler_receiver_tasks);
        scheduler.watch();
        WorkerManager {
            send_to_scheduler: send_to_scheduler
            //workers: workers,
            //watcher: watcher
        }
    }
    pub fn add_task(&self, task: WorkerSend) -> glib::Receiver<WorkerReply> {
        let (worker_sender, consumer_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        self.send_to_scheduler.send(SchedulerEvent::AddTask(WorkerOrder {order: task, result: worker_sender}));
        consumer_receiver
    }
}

pub struct WorkerScheduler {
    workers: HashMap<ThreadId, Worker>,
    queue: VecDeque<WorkerOrder>,
    scheduler_receiver_tasks: Receiver<SchedulerEvent>
}

impl WorkerScheduler {
    pub fn new(threads: u64, worker_sender_workerreply: glib::Sender<WorkerReply>, scheduler_receiver_tasks: Receiver<SchedulerEvent>) -> WorkerScheduler {
        let mut workers = HashMap::new();
        for i in 0..threads {
            let wsend_workerreply = worker_sender_workerreply.clone();
            let worker = Worker::new(WorkerOccupation::None, wsend_workerreply);
            workers.insert(worker.get_thread_id(), worker);
        }
        WorkerScheduler {
            workers: workers,
            queue: VecDeque::new(),
            scheduler_receiver_tasks: scheduler_receiver_tasks
        }
    }
    pub fn watch(mut self) {
        thread::spawn(move || {
            thread::sleep_ms(500); // wait for all threads to start
            loop {
                match self.scheduler_receiver_tasks.recv().unwrap() {
                    SchedulerEvent::AddTask(order) => {
                        if self.queue.len() > 0 {
                            self.queue.push_back(order);
                            for (thread_id, worker) in &mut self.workers {
                                if worker.busy == false { // idle worker found
                                    if let Some(order) = self.queue.pop_front() {
                                        debug!("Queue but idle worker found");
                                    worker.sender_to_worker.send(order);
                                    worker.busy = true;
                                    break;
                                    }
                                }
                            }
                        } else { // try to find an idle worker first
                            let mut assigned_task_to_worker = false;
                            'try_find_idle_thread: for (thread_id, worker) in &mut self.workers {
                                if worker.busy == false { // idle worker found
                                debug!("No queue and idle worker found");
                                    worker.sender_to_worker.send(order.clone());
                                    worker.busy = true;
                                    assigned_task_to_worker = true;
                                    break 'try_find_idle_thread;
                                }
                            }
                            // idle worker not found, queue now, a space will be available later
                            if assigned_task_to_worker == false {
                                debug!("No queue but no idle workers");
                                self.queue.push_back(order);
                            }
                        }
                    },
                    SchedulerEvent::WorkerIsBusy(thread_id) => {
                        self.workers.get_mut(&thread_id).unwrap().busy = true;
                    },
                    SchedulerEvent::WorkerIsFree(thread_id) => {
                        if let Some(order) = self.queue.pop_front() {
                            debug!("Worker {:?} was free but not anymore", &thread_id);
                            self.workers.get(&thread_id).unwrap().sender_to_worker.send(order);
                        } else {
                            debug!("Worker {:?} is free", &thread_id);
                            self.workers.get_mut(&thread_id).unwrap().busy = false;
                        }
                    }
                }
            }
        });
    }
}
