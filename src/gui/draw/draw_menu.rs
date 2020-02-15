use crate::game::State;
use super::fill_rect::fill_rect;

pub fn draw_menu(state: State) {
  let canvas = state.canvas.unwrap();
  let ctx = canvas.ctx;
  ctx.set_fill_style_color("black");

  let start_button = state.menu.start_button;
  ctx.set_fill_style_color("red");
  fill_rect(&ctx, start_button.x, start_button.y, start_button.width, start_button.height);
}
