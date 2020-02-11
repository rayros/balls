#[macro_use]
extern crate stdweb;
mod store;
mod gui;
mod story;
mod canvas;
use story::{Story, get_story};
use crate::store::{get_store, Store, Action};
use stdweb::web::{window, event::ResizeEvent, IEventTarget};
mod throttle;

fn watch_resize_event(story: Story) {
  window().add_event_listener({
    move |_: ResizeEvent| {
      story.borrow_mut().story(Action::WindowResize);
    }
  });
}


fn main() {
  stdweb::initialize();
  let store: Store = get_store();
  let story: Story = get_story(store);

  watch_resize_event(story.clone());
  // game_loop(story.clone());
  story.borrow().story(Action::None);
  stdweb::event_loop();
}
