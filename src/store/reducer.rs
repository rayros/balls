use crate::store::Action;
use crate::store::State;

pub fn reducer(state: &State, action: &Action) -> State {
  match action {
    Action::None => State {
      action_text: "none".to_string(),
      ..state.clone()
    },
    // Action::Move { counter } => State {
    //   action_text: "move".to_string(),
    //   counter: *counter,
    //   ..state.clone()
    // },
    Action::FontLoaded => State {
      action_text: "font_loaded".to_string(),
      ..state.clone()
    },
    Action::NewCanvas { canvas, width, height } => State {
      action_text: "new_canvas".to_string(),
      canvas: Some(canvas.clone()),
      canvas_width: *width,
      canvas_height: *height,
      ..state.clone()
    },
    Action::Draw => State {
      action_text: "draw_menu".to_string(),
      ..state.clone()
    },
    Action::WindowResize => state.clone(),
    Action::CanvasResize { width, height } => State {
      canvas_width: *width,
      canvas_height: *height,
      ..state.clone()
    }
  }
}