extern crate rand;

use crate::store::state::Ball;
use crate::store::state::Board;
use rand::prelude::*;

pub fn find_place_for_ball(board: Board) -> (usize, usize) {
  let mut rng = thread_rng();
  let random_column: usize = rng.gen_range(0, 9);
  let random_row: usize = rng.gen_range(0, 9);
  // let board = board.clone();
  let ball: &Ball = &board[random_row][random_column];
  if ball.num != 0 {
    for row_index in random_row..board.len() {
      for column_index in random_column..board[row_index].len() {
        let next_ball = &board[row_index][column_index];
        if next_ball.num == 0 {
          return (row_index, column_index);
        }
      } 
    }
    for row_index in 0..random_row {
      for column_index in 0..random_column {
        let next_ball = &board[row_index][column_index];
        if next_ball.num == 0 {
          return (row_index, column_index);
        }
      } 
    }
  }
  return (random_row, random_column);
}

pub fn get_balls(board: Board) -> Vec<Ball> {
  let mut vec = vec![];
  for row_index in 0..board.len() {
    for column_index in 0..board[row_index].len() {
      vec.push(board[row_index][column_index].clone());
    } 
  }
  vec.clone()
}

pub fn gen_ball_number() -> u8 {
  let mut rng = thread_rng();
  rng.gen_range(1, 7)
}