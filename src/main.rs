#[macro_use]
extern crate stdweb;
mod store;
mod gui;
mod story;
mod canvas;
use story::{Story, get_story};
use crate::store::{get_store, Store, Action};
use stdweb::web::{window, event::ResizeEvent, IEventTarget};

fn game_loop(story: Story) {
  let story = story.clone();
  story.borrow_mut().story(Action::Draw);
  window().request_animation_frame(|_: f64| {
    game_loop(story);
  });
}

fn main() {
  stdweb::initialize();
  let store: Store = get_store();
  let story: Story = get_story(store);
  window().add_event_listener({
    let story = story.clone();
    move |_: ResizeEvent| {
      story.borrow_mut().story(Action::WindowResize);
    }
  });
  game_loop(story.clone());
  story.borrow().story(Action::None);
  // let _handle = fonts::load("Amatica SC", "local('Amatica SC'), url(https://fonts.gstatic.com/s/amaticasc/v1/r4IyjqPgTL10SERuDuUzpAzyDMXhdD8sAj6OAJTFsBI.woff2) format('woff2')", None)
  //   .unwrap()
  //   .done(move |result: Result<Value, Value>| match result {
  //     Ok(value) => {
  //       let store = store.clone();
  //       store.borrow_mut().dispatch(Action::Move { counter: 3 });
  //       console!(log, value);
  //     }
  //     Err(error) => {
  //       console!(error, error);
  //     }
  //   })
  //   .leak();
  stdweb::event_loop();
}
