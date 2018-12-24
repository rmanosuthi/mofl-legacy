use mogame::mogame;
use std::path::PathBuf;
// use std::io::prelude::*;
use momod::momod;

pub struct moenv {
    folder_layout: Vec<String>,
    games: Vec<mogame>,
    base_path: PathBuf
}
impl moenv {
    pub fn new() -> moenv {
        moenv {
            folder_layout: Vec::new(),
            games: Vec::new(),
            base_path: PathBuf::from("~/.config/mofl")
        }
    }
    pub fn get_base_path(&self) -> PathBuf {
        return self.base_path.to_owned();
    }
    pub fn set_base_path(&mut self, input: PathBuf) -> () {
        for game in &mut self.games {
            game.update_base_path(input.to_owned());
        }
    }
    pub fn get_game(&self, index: usize) -> &mogame {
            return self.games.get(index).expect("Index out of bounds");
    }
    pub fn add_game(&mut self, game: mogame) -> () {
        self.games.push(game);
    }
}