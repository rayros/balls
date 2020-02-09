use crate::canvas::Canvas;

#[derive(Clone)]
pub enum Action {
  None,
  FontLoaded,
  NewCanvas { canvas: Canvas, width: u32, height: u32 },
  Draw,
  WindowResize,
  CanvasResize { width: u32, height: u32 },
  Click
  // Move { counter: u8 }
}

impl PartialEq for Action  {
  fn eq(&self, other: &Self) -> bool {
      self == other
  }
}