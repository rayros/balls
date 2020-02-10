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
pub struct Menu {
  start_button: Button
}

#[derive(Default, Clone)]
pub struct Button {
  x: u32,
  y: u32,
  width: u32,
  height: u32
}

#[derive(Default, Clone)]
pub struct State {
  pub view: View,
  pub menu: Menu,
  pub board: Vec<u8>,
  pub counter: u8,
  pub canvas: Option<Canvas>,
  pub canvas_width: u32,
  pub canvas_height: u32
}