extern crate rand;

use crate::store::state::Game;
use crate::store::state::Ball;
use crate::store::state::Board;
use rand::prelude::*;


pub fn find_place_for_ball(board: Board) -> Option<(usize, usize)> {
  let mut rng = thread_rng();
  let random_column: usize = rng.gen_range(0, 9);
  let random_row: usize = rng.gen_range(0, 9);
  // let board = board.clone();
  let maybe_ball: Option<Ball> = board[random_row][random_column].clone();
  match maybe_ball {
    Some(_ball) => {
      let mut result = None;
      for row_index in random_row..board.len() {
        for column_index in random_column..board[row_index].len() {
          let maybe_ball = &board[row_index][column_index];
          match maybe_ball {
            None => {
              result = Some((row_index, column_index));
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
              result = Some((row_index, column_index));
              break;
            },
            Some(_ball) => {}
          }
        } 
      }
      result
    },
    None => Some((random_row, random_column))
  }
}

pub fn get_balls(board: Board) -> Vec<Ball> {
  let mut vec = vec![];
  for row_index in 0..board.len() {
    for column_index in 0..board[row_index].len() {
      let maybe_ball = board[row_index][column_index].clone();
      match maybe_ball {
        Some(ball) => {
          vec.push(ball);
        },
        None => {}
      }
    } 
  }
  vec
}

pub fn gen_ball_number() -> u8 {
  let mut rng = thread_rng();
  rng.gen_range(1, 7)
}

pub fn get_position_for_ball(game: Game, place: (usize, usize)) -> (f64, f64) {
  // console!(log, row_index as u32);
  // console!(log, column_index as u32);
  let x = game.board_x + game.cell_width / 2 + game.line_width / 4 + place.1 as i32 * game.cell_width;
  let y = game.board_y + game.cell_width / 2 + game.line_width / 4 + place.0 as i32 * game.cell_width;
  (f64::from(x), f64::from(y))
}