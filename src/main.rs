// 3rd party crates/modules
extern crate serde;
extern crate serde_json;

use std::io;
use std::fs::File;
use std::io::Read;

// Local crates/modules
use chess::controller::Controller;
use chess::game::{GameState, GameStateResult, State};
use chess::move_logger::LoggedMove;
use chess::piece_move::PieceMove;
use chess::pieces::piece::Piece;
use chess::position::Position;

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

  let mut controller = Controller::new(game_config);

  let mut game_state_result: GameStateResult;
    match controller.initialise_game() {
    Ok(result)  => game_state_result = result,
    Err(e) => {
      println!("Unable to initialise game. Reason: {}", e);
      return;
    }
  }
  
  // Print initial state of the game
  print_unicode_board(&game_state_result.board, game_state_result.game_state.clone());
    
  // Loop the turn based logic until there is an outcome for the game
  while let State::Active = game_state_result.game_state.state {
    // Get next move from terminal
    let piece_move = get_move_input();
    
    // Call the controller to process the move
    match controller.process_move(piece_move){
      Ok(result)  => {
        game_state_result = result;

        // Print the move log
        print_logged_moves(game_state_result.move_log);

        // Print state of the game
        print_unicode_board(&game_state_result.board, game_state_result.game_state.clone());
      },
      Err(e) => {
        println!("Unable to process move. Reason: {}", e);
      }
    }
  }

  println!("");
  println!("Game Over!");
}

/**
 * A function to retrieve the input of the next move from the terminal input.
 * Loops on the input string until it is valid
 * Scholars mate:
 *  4,1,4,3   pawn e4   (w)
 *  4,6,4,4   pawn e5   (b)
 *  5,0,2,3   bishop c4 (w)
 *  1,7,2,5   kngiht c6 (b)
 *  3,0,7,4   queen h5  (w)
 *  6,7,5,5   knight f6 (b)
 *  7,4,5,6   queen f7  (w) *checkmate*
 */
fn get_move_input() -> PieceMove {
  loop {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(_goes_into_input_above) => {},
      Err(_no_updates_is_fine) => {},
    }
    input = input.trim().to_string();

    let split: Vec<&str> = input.split(",").collect();
    if split.len() == 4 || split.len() == 5 {
      let mut promotion = None;
      if split.len() == 5 {
        promotion = Some(split[4].to_string());
      }
      return PieceMove {
        start: Position {
          row: split[1].parse::<usize>().unwrap(), 
          column: split[0].parse::<usize>().unwrap()
        }, 
        end: Position {
          row: split[3].parse::<usize>().unwrap(), 
          column: split[2].parse::<usize>().unwrap()
        },
        promotion
      }
    }
    println!("Invalid Input");
  }
}

/**
 * 
 */
fn print_logged_moves(moves: Vec<Vec<LoggedMove>>) {
  for i in 0..moves.len() {
    for j in 0..moves[i].len() {
      print!("{} ", moves[i][j].pgn_notation);
    }
    println!("");
  }
}

/**
 * 
 */
fn print_unicode_board(board: &Vec<Vec<Option<Piece>>>, game_state: GameState) {
  println!("");
  println!("   0 1 2 3 4 5 6 7");

  let mut row_index = 7;

  for row in board.iter().rev() {
    print!("{}  ", row_index);
    for col in row.iter() {
      if col.is_some() {
        let piece = col.as_ref().unwrap();
        match &piece {
          Piece::Bishop(is_white) => {
            if *is_white { print!("♗ ") } else { print!("♝ ") };
          },
          Piece::King(is_white) => {
            if *is_white{ print!("♔ ") } else { print!("♚ ") };
          },
          Piece::Knight(is_white) => {
            if *is_white { print!("♘ ") } else { print!("♞ ") };
            },
          Piece::Pawn(is_white) => {
            if *is_white { print!("♙ ") } else { print!("♟︎ ") };
          },
          Piece::Queen(is_white) => {
            if *is_white { print!("♕ ") } else { print!("♛ ") };
          },
          Piece::Rook(is_white) => {
            if *is_white { print!("♖ ") } else { print!("♜ ") };
          },
        }
        
      } else {
        print!("- ");
      }        
    }
    println!(" {}", row_index);
    row_index -= 1;
  }

  println!("   0 1 2 3 4 5 6 7");

  // TODO: clear console between updates
  println!("");
  if game_state.white_turn {
    println!("Turn: White");
  } else {
    println!("Turn: Black");
  }
  println!("Game State: {:?}", game_state.state);
  println!("In Check? {:?}", game_state.in_check);
  println!("");
  println!("#################################");
  println!("");
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