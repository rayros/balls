extern crate rand;

use crate::store::state::Board;
use rand::prelude::*;

pub fn find_place_for_ball(board: Board) -> (usize, usize) {
  let mut rng = thread_rng();
  let random_column: usize = rng.gen_range(0, 9);
  let random_row: usize = rng.gen_range(0, 9);
  let value: u8 = board[random_row][random_column];
  if value != 0 {
    for row_index in random_row..board.len() {
      for column_index in random_column..board[row_index].len() {
        let next_value = board[row_index][column_index];
        if next_value == 0 {
          return (row_index, column_index);
        }
      } 
    }
    for row_index in 0..random_row {
      for column_index in 0..random_column {
        let next_value = board[row_index][column_index];
        if next_value == 0 {
          return (row_index, column_index);
        }
      } 
    }
  }
  return (random_row, random_column);
}