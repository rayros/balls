use crate::canvas::Canvas;
use crate::store::{State, View};
mod draw_menu;
use draw_menu::draw_menu;

pub fn draw(state: State) {
  // console!(log, store.view.to_string());
  let canvas = state.canvas.clone();
  match canvas {
    Some(canvas) => {
      match state.view {
        View::Menu => draw_menu(state.clone()),
        View::Game => draw_game(canvas)
      }
    },
    None => {}
  }
}

fn draw_game(canvas: Canvas) {
  let ctx = canvas.ctx;
  ctx.set_fill_style_color("white");
  ctx.fill_rect(
    f64::from(10),
    f64::from(10),
    f64::from(canvas.element.width() - 20),
    f64::from(canvas.element.height() - 20),
  );
}