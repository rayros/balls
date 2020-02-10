use crate::store::state::Button;
use crate::store::state::Menu;
use crate::store::Action;
use crate::store::State;

pub fn reducer(state: &State, action: &Action) -> State {
  match action {
    Action::None => state.clone(),
    Action::FontLoaded => state.clone(),
    Action::Draw => state.clone(),
    Action::WindowResize => state.clone(),
    Action::Click { x: _, y: _ } => state.clone(),
    Action::CanvasResize { width, height } => State {
      canvas_width: *width,
      canvas_height: *height,
      ..state.clone()
    },
    Action::NewCanvas { canvas, width, height } => State {
      canvas: Some(canvas.clone()),
      canvas_width: *width,
      canvas_height: *height,
      menu: construct_menu(*width, *height),
      ..state.clone()
    },
    Action::ChangeView { view } => State {
      view: view.clone(),
      ..state.clone()
    }
  }
}

pub fn construct_menu(canvas_width: u32, _: u32) -> Menu {
  let width = canvas_width - 50;
  Menu {
    start_button: Button {
      x: 25,
      y: 25,
      width,
      height: 50
    }
  }
}