use crate::canvas::Canvas;
use std::fmt;

#[derive(Debug, Clone)]
pub enum View {
  Menu,
  Game,
}

impl Default for View {
  fn default() -> Self {
    View::Menu
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
pub struct State {
  pub view: View,
  pub menu: Menu,
  pub board: Vec<u8>,
  pub counter: u8,
  pub canvas: Option<Canvas>,
  pub canvas_width: i32,
  pub canvas_height: i32,
}
