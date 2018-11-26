pub struct momod {
    pub label: String,
    pub load_order: isize,
    pub nexus_id: isize,
    pub dir: String,
}
impl momod {

}
    pub fn new() -> momod {
        momod {
            label: String::from(""),
            load_order: -1,
            nexus_id: -1,
            dir: String::from("")
        }
    }