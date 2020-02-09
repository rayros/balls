use crate::store::{State, View};

pub fn draw(store: State) {
  // console!(log, store.view.to_string());
  match store.canvas {
    Some(canvas) => {
      let ctx = canvas.ctx;
      ctx.set_fill_style_color("black");
      ctx.fill_rect(
        f64::from(10),
        f64::from(10),
        f64::from(canvas.element.width() - 20),
        f64::from(canvas.element.height() - 20),
      );
      match store.view {
        View::Menu => {
        
        },
        View::Game => {
          
        }
      }
    },
    None => {}
  }
}