use crate::game::find_path::Path;
use crate::game::state::Ball;
use crate::game::view::View;
use crate::canvas::Canvas;

#[derive(Clone)]
pub enum Action {
  None,
  FontLoaded,
  NewCanvas { canvas: Canvas, width: i32, height: i32 },
  Draw,
  WindowResize,
  CanvasResize { width: i32, height: i32 },
  Click { x: i32, y: i32 },
  ChangeView { view: View },
  AddBalls,
  SelectBall { maybe_ball: Option<Ball> },
  ChangeSelectedBallColor { ball: Ball },
  MoveBall { path: Path },
  CheckLines,
  NewGame
}

impl PartialEq for Action  {
  fn eq(&self, other: &Self) -> bool {
      self == other
  }
}