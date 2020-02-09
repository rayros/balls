mod canvas;
use crate::store::Action;
use crate::story::{Story};
pub use canvas::Canvas;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{window};
use stdweb::web::{event::ClickEvent, IEventTarget};

pub fn resize_canvas_to_window_size(canvas: &CanvasElement) -> (u32, u32) {
  let width = window().inner_width() as u32;
  let height = window().inner_height() as u32;
  canvas.set_width(window().inner_width() as u32);
  canvas.set_height(window().inner_height() as u32);
  (width, height)
}

pub fn watch_click_event(story: Story, canvas: Canvas) {
  canvas.element.add_event_listener({
    let story = story.clone();
    move |_: ClickEvent| {
      story.borrow_mut().story(Action::Click);
    }
  });
}


