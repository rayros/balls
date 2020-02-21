#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::too_many_lines)]
#[macro_use]
extern crate stdweb;
mod store;
mod gui;
mod story;
mod canvas;
mod game;
use crate::story::{Story};
use crate::game::{reducer, Action};
use crate::store::{get_store};
use stdweb::web::{window, event::ResizeEvent, IEventTarget};
mod throttle;

fn watch_resize_event(story: Story) {
  window().add_event_listener({
    move |_: ResizeEvent| {
      story.borrow_mut().story(Action::WindowResize);
    }
  });
}

// TODO new ResizeObserver(outputsize).observe(textbox)

fn main() {
  stdweb::initialize();
  let story: Story = story::get(get_store(reducer));

  watch_resize_event(story.clone());
  story.borrow().story(Action::None);
  stdweb::event_loop();
}
