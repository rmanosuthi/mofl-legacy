use mogame::Game;
use std::path::PathBuf;
// use std::io::prelude::*;
use momod::momod;

pub struct Environment {
    folder_layout: Vec<String>,
    games: Vec<Game>,
    base_path: PathBuf
}
impl Environment {
    pub fn new() -> Environment {
        Environment {
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
    pub fn get_game(&self, index: usize) -> &Game {
            return self.games.get(index).expect("Index out of bounds");
    }
    pub fn add_game(&mut self, game: Game) -> () {
        self.games.push(game);
    }
}