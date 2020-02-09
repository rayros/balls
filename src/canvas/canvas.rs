use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};
use stdweb::traits::*;
use stdweb::unstable::TryInto;

#[derive(Clone)]
pub struct Canvas {
  selector: String,
  pub element: CanvasElement,
  pub ctx: CanvasRenderingContext2d,
}

impl Canvas {
  pub fn new(canvas_attr_id: &str) -> Canvas {
    let element: CanvasElement = document()
      .query_selector(canvas_attr_id)
      .unwrap()
      .unwrap()
      .try_into()
      .unwrap();

    let ctx: CanvasRenderingContext2d = element.get_context().unwrap();

    Canvas {
      selector: String::from(canvas_attr_id),
      element,
      ctx
    }
  }
}
