use std::io;

use chess::{
  controller::Controller,
  position::Position, 
  piece_move::PieceMove
};

pub struct IOController {
  white_human: bool,
  black_human: bool
  //ai_engine: A
}

impl IOController {
  pub fn new(white_human: bool, black_human: bool) -> Self {
    Self {
      white_human,
      black_human
    }
  }

  /**
   * Retrieves the next move for white 
   */
  fn get_white_move(&self) -> PieceMove {
    if self.white_human {
      return get_move_input();
    } else {
      return get_move_input(); // TODO: Change later for ai implementation
    }
  }

  /**
   * Retrieves the next move for black 
   */
  fn get_black_move(&self) -> PieceMove {
    if self.black_human {
      return get_move_input();
    } else {
      return get_move_input(); // TODO: Change later for ai implementation
    }
  }
}

impl Controller for IOController {
  /**
   * Retrieves the next chosen move from the white or black player based on
   * the provided white_turn bool parameter
   */
  fn get_move(&self, white_turn: bool) -> PieceMove {
    if white_turn {
      return self.get_white_move();
    } else {
      return self.get_black_move();
    }
  }
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