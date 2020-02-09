use crate::canvas::Canvas;

#[derive(Clone)]
pub enum View {
  Menu
}

impl Default for View {
  fn default() -> Self { View::Menu }
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