use stdweb::web::CanvasRenderingContext2d;
use super::fill_rect::fill_rect;

pub trait ClearCtx {
  fn clear(&self, width: i32, height: i32);
}

impl ClearCtx for CanvasRenderingContext2d {
  fn clear(&self, width: i32, height: i32) {
    self.set_fill_style_color("white");
    fill_rect(self, 0, 0, width, height);
  }
}
