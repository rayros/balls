mod canvas;
use crate::game::Action;
use crate::story::{Story};
pub use canvas::Canvas;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{window};
use stdweb::web::{event::ClickEvent, IEventTarget};
use stdweb::traits::IMouseEvent;

pub fn resize_canvas_to_window_size(canvas: &CanvasElement) -> (i32, i32) {
  let width = window().inner_width();
  let height = window().inner_height();
  canvas.set_width(window().inner_width() as u32);
  canvas.set_height(window().inner_height() as u32);
  (width, height)
}

pub fn watch_click_event(story: Story, canvas: Canvas) {
  canvas.element.add_event_listener({
    move |event: ClickEvent| {
      let x = event.client_x() as i32;
      let y = event.client_y() as i32;
      story.borrow_mut().story(Action::Click { x, y });
    }
  });
}


