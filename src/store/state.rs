use crate::canvas::Canvas;
use std::fmt;

#[derive(Debug, Clone)]
pub enum View {
  None,
  Menu,
  Game,
}

impl Default for View {
  fn default() -> Self {
    View::None
  }
}

impl fmt::Display for View {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({:?})", self)
  }
}

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
