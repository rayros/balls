#[macro_use]
extern crate stdweb;
mod store;
mod gui;
mod story;
mod canvas;
mod game;
use crate::story::{Story, get_story};
use crate::game::{State, reducer, Action};
use crate::store::{get_store, Store};
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
  let store: Store<State, Action> = get_store(reducer);
  let story: Story = get_story(store);

  watch_resize_event(story.clone());
  story.borrow().story(Action::None);
  stdweb::event_loop();
}
