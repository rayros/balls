extern crate rand;

use crate::store::state::Place;
use crate::store::state::Game;
use crate::store::state::Ball;
use crate::store::state::Board;
use rand::prelude::*;


pub fn find_place_for_ball(board: Board) -> Option<Place> {
  let mut rng = thread_rng();
  let random_column: usize = rng.gen_range(0, 9);
  let random_row: usize = rng.gen_range(0, 9);
  let maybe_ball: Option<Ball> = board[random_row][random_column].clone();
  match maybe_ball {
    None => Some(Place { row_index: random_row, column_index: random_column }),
    Some(_ball) => {
      let mut place = None;
      for row_index in random_row..board.len() {
        for column_index in random_column..board[row_index].len() {
          let maybe_ball = &board[row_index][column_index];
          match maybe_ball {
            None => {
              place = Some(Place { row_index, column_index });
              break;
            },
            Some(_ball) => {}
          }
        } 
      }
      for row_index in 0..random_row {
        for column_index in 0..random_column {
          let maybe_ball = &board[row_index][column_index];
          match maybe_ball {
            None => {
              place = Some(Place { row_index, column_index });
              break;
            },
            Some(_ball) => {}
          }
        } 
      }
      place
    }
  }
}

pub fn get_balls(board: Board) -> Vec<Ball> {
  let mut vec = vec![];
  for board_rows in board.iter() {
    for maybe_ball in board_rows.iter() {
      if let Some(ball) = maybe_ball {
        vec.push(ball.clone());
      }
    }
  }
  vec
}

pub fn gen_ball_number() -> u8 {
  let mut rng = thread_rng();
  rng.gen_range(1, 7)
}

pub fn get_position_for_ball(game: Game, place: Place) -> (i32, i32) {
  // console!(log, row_index as u32);
  // console!(log, column_index as u32);
  let x = game.board_x + game.cell_width / 2 + game.line_width / 4 + place.column_index as i32 * game.cell_width;
  let y = game.board_y + game.cell_width / 2 + game.line_width / 4 + place.row_index as i32 * game.cell_width;
  (x, y)
}