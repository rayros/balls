use crate::canvas::Canvas;

#[derive(Clone)]
pub enum Action {
  None,
  FontLoaded,
  NewCanvas { canvas: Canvas, width: u32, height: u32 },
  DrawMenu,
  WindowResize,
  CanvasResize { width: u32, height: u32 },
  // Move { counter: u8 }
}

impl PartialEq for Action  {
  fn eq(&self, other: &Self) -> bool {
      self == other
  }
}