use stdweb::web::CanvasRenderingContext2d;
use crate::store::State;

fn fill_rect(ctx: CanvasRenderingContext2d, x: i32, y: i32, width: i32, height: i32) {
  ctx.fill_rect(
    f64::from(x),
    f64::from(y),
    f64::from(width),
    f64::from(height),
  );
}

pub fn draw_menu(state: State) {
  let canvas = state.canvas.unwrap();
  let ctx = canvas.ctx;
  ctx.set_fill_style_color("black");

  let start_button = state.menu.start_button;
  ctx.set_fill_style_color("red");
  fill_rect(ctx, start_button.x, start_button.y, start_button.width, start_button.height);
}
