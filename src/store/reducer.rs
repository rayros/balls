use crate::store::state::Ball;
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
    Action::Click { .. } => state.clone(),
    Action::CanvasResize { width, height } => {
      let state = State {
        canvas_width: *width,
        canvas_height: *height,
        ..state.clone()
      };
      match state.view {
        View::Game => State {
          game: resize_game(state.clone()),
          ..state
        },
        View::Menu => State {
          menu: resize_menu(state.clone()),
          ..state
        },
        View::None => state
      }
    },
    Action::NewCanvas { canvas, width, height } => State {
      canvas: Some(canvas.clone()),
      canvas_width: *width,
      canvas_height: *height,
      ..state.clone()
    },
    Action::AddBalls => State {
      game: add_balls(state.clone()),
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

// fn update_balls_postions() {}

fn resize_game(state: State) -> Game {
  let State {
    canvas_height,
    canvas_width,
    game: Game { board, balls, .. },
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
  let line_width = board_width / 100;
  let cell_width = board_width / 9;
  Game {
    board,
    balls,
    board_width,
    line_width,
    cell_width,
    board_x,
    board_y
  }
}

use super::selectors::*;

fn add_balls(state: State) -> Game {
  let State {
    game,
    ..
  } = state;
  let (row_index, column_index) = find_place_for_ball(game.board.clone());
  let mut board = game.board.clone();
  let ball = Ball {
    num: gen_ball_number(),
    position: get_position_for_ball(game.clone(), row_index, column_index)
  };
  board[row_index][column_index] = ball;
  let balls = get_balls(board.clone());
  Game {
    board,
    balls,
    ..game
  }
}
