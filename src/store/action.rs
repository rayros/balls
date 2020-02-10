use crate::store::state::View;
use crate::canvas::Canvas;

#[derive(Clone)]
pub enum Action {
  None,
  FontLoaded,
  NewCanvas { canvas: Canvas, width: u32, height: u32 },
  Draw,
  WindowResize,
  CanvasResize { width: u32, height: u32 },
  Click { x: i32, y: i32 },
  ChangeView { view: View }
  // Move { counter: u8 }
}

impl PartialEq for Action  {
  fn eq(&self, other: &Self) -> bool {
      self == other
  }
}