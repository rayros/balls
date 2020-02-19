extern crate rand;

use crate::game::state::Ball;
use crate::game::state::Board;
use crate::game::state::Game;
use crate::game::state::Place;
use rand::prelude::*;

pub fn get_privacy_policy_link() -> String {
  String::from("https://studiolacosanostra.github.io/projects/lines/privacy_policy.html")
}

pub fn equal_place(place_a: Place, place_b: Place) -> bool {
  place_a.row_index == place_b.row_index && place_a.column_index == place_b.column_index
}

fn board_to_empty_places(board: &Board) -> Vec<Place> {
  let mut vec: Vec<Place> = vec![];
  for (row_index, row) in board.iter().enumerate() {
    for (column_index, column) in row.iter().enumerate() {
      if column.is_none() {
        vec.push(Place { row_index, column_index });
      }
    }
  }
  vec
}

pub fn find_place_for_ball(board: Board) -> Option<Place> {
  let empty_places = board_to_empty_places(&board);
  // console!(log, empty_places.clone());
  let place = empty_places.choose(&mut rand::thread_rng());
  place.cloned()
}

pub fn get_balls(board: &Board) -> Vec<Ball> {
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

pub fn maybe_place_intersect(game: &Game, x: i32, y: i32) -> Option<Place> {
  let start_x = x - game.board_x;
  let start_y = y - game.board_y;
  if start_x > 0 && start_y > 0 {
    let column_index = f64::from(start_x / game.cell_width).floor() as usize;
    let row_index = f64::from(start_y / game.cell_width).floor() as usize;
    if row_index < game.board.len() && column_index < game.board[row_index].len() {
      return Some(Place { row_index, column_index });
    }
  }
  None
}

pub fn get_position_for_ball(game: &Game, place: &Place) -> (i32, i32) {
  // console!(log, row_index as u32);
  // console!(log, column_index as u32);
  let x = game.board_x
    + game.cell_width / 2
    + game.line_width / 4
    + place.column_index as i32 * game.cell_width;
  let y = game.board_y
    + game.cell_width / 2
    + game.line_width / 4
    + place.row_index as i32 * game.cell_width;
  (x, y)
}
