use crate::game::Action;
use crate::story::{Story};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{window};
use stdweb::web::{event::ClickEvent};
use stdweb::traits::IMouseEvent;
use stdweb::web::{document, CanvasRenderingContext2d};
use stdweb::traits::*;
use stdweb::unstable::TryInto;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn resize_canvas_to_window_size(canvas: &CanvasElement) -> (i32, i32) {
  let width = window().inner_width();
  let height = window().inner_height();
  canvas.set_width(window().inner_width() as u32);
  canvas.set_height(window().inner_height() as u32);
  (width, height)
}

pub fn watch_click_event(story: Story, canvas: &Canvas) {
  canvas.element.add_event_listener({
    move |event: ClickEvent| {
      let x = event.client_x() as i32;
      let y = event.client_y() as i32;
      story.borrow_mut().story(Action::Click { x, y });
    }
  });
}


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
