use crate::canvas::Canvas;
use std::fmt;

#[derive(Debug, Clone)]
pub enum View {
  Menu,
  Game
}

impl Default for View {
  fn default() -> Self { View::Menu }
}

impl fmt::Display for View {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({:?})", self)
  }
}

#[derive(Default, Clone)]
pub struct State {
  pub action_text: String,
  pub view: View,
  pub board: Vec<u8>,
  pub counter: u8,
  pub canvas: Option<Canvas>,
  pub canvas_width: u32,
  pub canvas_height: u32
}