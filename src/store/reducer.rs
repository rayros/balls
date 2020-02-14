use crate::store::state::Place;
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
    },
    Action::SelectBall { ball } => select_ball(state.clone(), ball.clone())
  }
}

fn select_ball(state: State, ball: Ball) -> State {
  let State {
    game,
    ..
  } = state;
  let mut board = game.board.clone();
  let selected_ball = game.selected_ball.clone();
  if let Some(selected_ball) = selected_ball {
    board[selected_ball.place.row_index][selected_ball.place.column_index] = Some(selected_ball.clone());
  }
  board[ball.place.row_index][ball.place.column_index] = None;
  let balls = get_balls(board.clone());
  let game = Game {
    board,
    selected_ball: Some(ball),
    balls,
    ..game
  };
  State {
    game,
    ..state
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

fn update_balls(game: Game) -> Game {
  let mut board = game.board.clone();
  for row_index in 0..board.len() {
    for column_index in 0..board[row_index].len() {
      let maybe_ball = board[row_index][column_index].clone();
      match maybe_ball {
        Some(ball) => {
          board[row_index][column_index] = Some(Ball {
            radius: game.cell_width / 2 / 5 * 4,
            position: get_position_for_ball(game.clone(), Place { row_index, column_index }),
            ..ball
          });
        },
        None => {}
      }
    } 
  }
  let selected_ball = match game.clone().selected_ball {
    Some(ball) => Some(Ball {
      radius: game.cell_width / 2 / 5 * 4,
      position: get_position_for_ball(game.clone(), ball.clone().place),
      ..ball
    }),
    None => None
  };
  Game {
    board,
    selected_ball,
    ..game
  }
}

fn resize_game(state: State) -> Game {
  let State {
    canvas_height,
    canvas_width,
    game,
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
  // console!(log, balls[0].num);
  let game = Game {
    board_width,
    line_width,
    cell_width,
    board_x,
    board_y,
    ..game
  };
  let game = update_balls(game.clone());
  let balls = get_balls(game.board.clone());
  Game {
    balls,
    ..game
  }
}

use super::selectors::*;

fn add_balls(state: State) -> Game {
  let State {
    game,
    ..
  } = state;
  let maybe_find_place = find_place_for_ball(game.board.clone());
  match maybe_find_place {
    Some(place) => {
      let mut board = game.board.clone();
      let num = gen_ball_number();
      let ball = Ball {
        num,
        radius: game.cell_width / 2 / 5 * 4,
        place: place.clone(),
        position: get_position_for_ball(game.clone(), place.clone()),
      };
      board[place.clone().row_index][place.clone().column_index] = Some(ball);
      let balls = get_balls(board.clone());
      Game {
        board,
        balls,
        ..game
      }
    },
    None => Game {
      isGameOver: true,
      ..game
    }
  }
}
