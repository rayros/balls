mod fonts;
mod draw;
use crate::story::Story;
use crate::canvas;

pub use draw::draw;
pub use canvas::resize_canvas_to_window_size;

pub fn load_fonts(story: Story) {
  fonts::load(
    "Amatica SC",
    "local('Amatica SC'), url(https://fonts.gstatic.com/s/amaticasc/v1/r4IyjqPgTL10SERuDuUzpAzyDMXhdD8sAj6OAJTFsBI.woff2) format('woff2')",
    None,
    story
  );
}

pub fn create_canvas(canvas_attr_id: &str) -> (canvas::Canvas, u32, u32) {
  let canvas = canvas::Canvas::new(canvas_attr_id);
  let (width, height) = canvas::resize_canvas_to_window_size(&canvas.element);
  (canvas, width, height)
}

