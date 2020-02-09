mod canvas;
pub use canvas::Canvas;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{window};

pub fn resize_canvas_to_window_size(canvas: &CanvasElement) -> (u32, u32) {
  let width = window().inner_width() as u32;
  let height = window().inner_height() as u32;
  canvas.set_width(window().inner_width() as u32);
  canvas.set_height(window().inner_height() as u32);
  (width, height)
}

// pub fn clear(canvas: Canvas) {
//   canvas.ctx.set_fill_style_color("white");
//   canvas.ctx.fill_rect(
//     f64::from(0),
//     f64::from(0),
//     f64::from(canvas.canvas.width()),
//     f64::from(canvas.canvas.height()),
//   );
// }

