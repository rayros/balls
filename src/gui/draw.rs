use crate::store::{State, View};

pub fn draw(store: State) {
  console!(log, store.view.to_string());
  let canvas = store.canvas.unwrap();
  let ctx = canvas.ctx;
  ctx.set_fill_style_color("black");
  ctx.fill_rect(
    f64::from(0),
    f64::from(0),
    f64::from(canvas.element.width()),
    f64::from(canvas.element.height()),
  );
  match store.view {
    View::Menu => {

    },
    View::Game => {
      
    }
  }
}