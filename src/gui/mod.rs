mod fonts;
use crate::story::Story;
use crate::store::State;
use crate::canvas;

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

pub fn draw(store: State) {
  let canvas = store.canvas.unwrap();
  let ctx = canvas.ctx;
  ctx.set_fill_style_color("black");
  ctx.fill_rect(
    f64::from(0),
    f64::from(0),
    f64::from(canvas.element.width()),
    f64::from(canvas.element.height()),
  );
}