use crate::store::View;
use crate::canvas::Canvas;

#[derive(Default, Clone)]
pub struct Menu {
  pub start_button: Button,
}

#[derive(Default, Clone)]
pub struct Button {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

impl Button {
  pub fn intersect(&self, x: i32, y: i32) -> bool {
    self.x <= x && x <= self.x + self.width && self.y <= y && y <= self.y + self.height
  }
}

#[derive(Default, Clone)]
pub struct Game {
  pub board_x: i32,
  pub board_y: i32,
  pub board_width: i32,
  pub line_width: i32,
  pub cell_width: i32,
  pub board: [[u8; 9]; 9]
}

#[derive(Default, Clone)]
pub struct State {
  pub view: View,
  pub menu: Menu,
  pub game: Game,
  pub canvas: Option<Canvas>,
  pub canvas_width: i32,
  pub canvas_height: i32,
}
