// 3rd party crates/modules
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Read;

// Local crates/modules
mod io_controller;
mod io_view;

use crate::io_controller::IOController;
use crate::io_view::IOView;
use chess::game::Game;

const CONFIG_FILE_PATH: &str = "config.json";

fn main() {
  let game_config: chess::config::GameConfig;
  match read_json_file(CONFIG_FILE_PATH) {
    Ok(config) => game_config = config,
    Err(err) => {
      eprintln!("Error: {}", err);
      return;
    }
  }

  println!("Welcome to Chess!");
  println!("");

  let mut game = Game::new(
    IOController::new(true, true),
    IOView {use_unicode: true},
    game_config
  );

  game.run();
}

/**
 * Reads the GameConfig from a json file
 */
fn read_json_file(file_path: &str) -> Result<chess::config::GameConfig, Box<dyn std::error::Error>> {
  // Open the file in read-only mode
  let mut file = File::open(file_path)?;

  // Read the file contents into a string
  let mut json_str = String::new();
  file.read_to_string(&mut json_str)?;

  // Deserialize the JSON string into a GameConfig
  let game_config: chess::config::GameConfig = serde_json::from_str(&json_str)?;

  Ok(game_config)
}