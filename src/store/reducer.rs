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
      ..state.clone()
    },
    Action::ChangeView { view } => State {
      view: view.clone(),
      ..state.clone()
    }
  }
}