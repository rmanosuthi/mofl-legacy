use mogame::Game;
use std::path::PathBuf;
// use std::io::prelude::*;
use momod::Mod;

pub struct Environment {
    folder_layout: Vec<String>,
    games: Vec<Game>,
    base_path: PathBuf
}
impl Environment {
    /// Vreates a new Environment instance with default config folder.
    pub fn new() -> Environment {
        Environment {
            folder_layout: Vec::new(),
            games: Vec::new(),
            base_path: PathBuf::from("~/.config/mofl")
        }
    }
    /// Gets the base path for the environment.
    pub fn get_base_path(&self) -> PathBuf {
        return self.base_path.to_owned();
    }
    /// Sets the base path for the environment.
    pub fn set_base_path(&mut self, input: PathBuf) -> () {
        for game in &mut self.games {
            game.update_base_path(input.to_owned());
        }
    }
    /// Gets a reference to a game given an index number.
    pub fn get_game(&self, index: usize) -> &Game {
            return self.games.get(index).expect("Index out of bounds");
    }
    /// Adds a game to the environment's collection.
    pub fn add_game(&mut self, game: Game) -> () {
        self.games.push(game);
    }
}