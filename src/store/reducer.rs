use crate::store::state::Button;
use crate::store::state::Menu;
use crate::store::Action;
use crate::store::State;
use super::{View, Game};

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
    Action::ChangeView { view } => {
      match view {
        View::Game => {
          State {
            view: view.clone(),
            game: resize_game(state.clone()),
            ..state.clone()
          }
        },
        View::Menu => {
          State {
            view: view.clone(),
            menu: resize_menu(state.clone()),
            ..state.clone()
          }
        },
        _ => state.clone()
      }
    }
  }
}

fn resize_menu(state: State) -> Menu {
  let width = state.canvas_width - 50;
  Menu {
    start_button: Button {
      x: 25,
      y: 25,
      width,
      height: 50
    }
  }
}

fn resize_game(state: State) -> Game {
  let State {
    canvas_height,
    canvas_width,
    game: Game { board, .. },
    ..
  } = state;
  let navigation_height = 50;
  let max_board_height = canvas_height - navigation_height;
  let board_width = if max_board_height < canvas_width {
    max_board_height
  } else {
    canvas_width
  };
  let board_x = (canvas_width - board_width) / 2;
  let board_y = navigation_height + (canvas_height - navigation_height - board_width) / 2;
  Game {
    board,
    board_width,
    board_x,
    board_y
  }
}