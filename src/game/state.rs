use crate::game::view::View;
use crate::canvas::Canvas;
use serde::Serialize;

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

#[derive(Default, Serialize, Clone)]
pub struct Place {
  pub row_index: usize,
  pub column_index: usize,
}

#[derive(Default, Serialize, Clone)]
pub struct Ball {
  pub num: u8,
  pub radius: i32,
  pub position: (i32, i32),
  pub place: Place,
}

js_serializable!( Ball );

impl Ball {
  pub fn intersect(&self, x: i32, y: i32) -> bool {
    self.position.0 - self.radius <= x
      && x <= self.position.0 + self.radius
      && self.position.1 - self.radius <= y
      && y <= self.position.1 + self.radius
  }
}

pub type Board = [[Option<Ball>; 9]; 9];

#[derive(Default, Serialize, Clone)]
pub struct SelectedBall {
  pub ball: Ball,
  pub is_selected_color: bool
}

js_serializable!( SelectedBall );

#[derive(Default, Serialize, Clone)]
pub struct Game {
  pub board_x: i32,
  pub board_y: i32,
  pub board_width: i32,
  pub line_width: i32,
  pub cell_width: i32,
  pub board: Board,
  pub balls: Vec<Ball>,
  pub isGameOver: bool,
  pub selected_ball: Option<SelectedBall>
}

js_serializable!( Game );

#[derive(Default, Clone)]
pub struct State {
  pub view: View,
  pub menu: Menu,
  pub game: Game,
  pub canvas: Option<Canvas>,
  pub canvas_width: i32,
  pub canvas_height: i32,
}
