use crate::game::{State};
mod fill_rect;
mod draw_game;
mod traits;

use draw_game::draw_game;

pub fn draw(state: State) {
  let canvas = state.canvas.clone();
  if canvas.is_some() {
    draw_game(state);
  }
}

