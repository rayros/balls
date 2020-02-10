use crate::store::State;

pub fn draw_menu(state: State) {
  let canvas = state.canvas.unwrap();
  let ctx = canvas.ctx;
  ctx.set_fill_style_color("black");
  ctx.fill_rect(
    f64::from(10),
    f64::from(10),
    f64::from(canvas.element.width() - 20),
    f64::from(canvas.element.height() - 20),
  );
}
