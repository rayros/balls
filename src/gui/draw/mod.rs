use crate::game::{State, View};
mod fill_rect;
mod draw_game;
mod traits;

use draw_game::draw_game;

pub fn draw(state: State) {
  // console!(log, store.view.to_string());
  let canvas = state.canvas.clone();
  match canvas {
    Some(_canvas) => {
      match state.view {
        View::None => {},
        View::Game => draw_game(state)
      }
    },
    None => {}
  }
}

