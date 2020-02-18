use std::collections::HashMap;

mod fonts;
mod draw;
use crate::story::Story;
use crate::canvas;

pub use draw::draw;
pub use canvas::resize_canvas_to_window_size;

pub fn load_fonts(story: Story) {
  let mut descriptors = HashMap::new();
  descriptors.insert("weight", "400");
  descriptors.insert("style", "normal");
  // console!(log, "load font");
  fonts::load(
    "Roboto",
    "local('Roboto'), local('Roboto-Regular'), url('./assets/Roboto-Regular.ttf') format('truetype')",
    Some(descriptors),
    story
  );
}

pub fn create_canvas(canvas_attr_id: &str) -> (canvas::Canvas, i32, i32) {
  let canvas = canvas::Canvas::new(canvas_attr_id);
  let (width, height) = canvas::resize_canvas_to_window_size(&canvas.element);
  (canvas, width, height)
}

