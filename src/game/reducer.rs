use crate::game::state::Board;
use crate::game::action::Action;
use crate::game::state::Ball;
use crate::game::state::Button;
use crate::game::state::Game;
use crate::game::state::Menu;
use crate::game::state::Place;
use crate::game::state::SelectedBall;
use crate::game::state::State;
use crate::game::view::View;

use super::selectors::*;

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
        View::None => state,
      }
    }
    Action::NewCanvas {
      canvas,
      width,
      height,
    } => State {
      canvas: Some(canvas.clone()),
      canvas_width: *width,
      canvas_height: *height,
      ..state.clone()
    },
    Action::AddBalls => State {
      game: add_balls(state.clone()),
      ..state.clone()
    },
    Action::ChangeView { view } => match view {
      View::Game => State {
        view: view.clone(),
        game: resize_game(state.clone()),
        ..state.clone()
      },
      View::Menu => State {
        view: view.clone(),
        menu: resize_menu(state.clone()),
        ..state.clone()
      },
      _ => state.clone(),
    },
    Action::SelectBall { maybe_ball } => select_ball(state.clone(), maybe_ball.clone()),
    Action::ChangeSelectedBallColor { ball } => {
      match state.game.selected_ball.clone() {
        Some(selected_ball) => {
          if equal_place(selected_ball.clone().ball.place, ball.clone().place) == false {
            return state.clone();
          }
          let selected_ball = SelectedBall {
            is_selected_color: !selected_ball.is_selected_color,
            ..selected_ball
          };
          State {
            game: Game {
              selected_ball: Some(selected_ball),
              ..state.game.clone()
            },
            ..state.clone()
          }
        },
        None => state.clone()
      }
    } 
  }
}

fn select_ball(state: State, maybe_ball: Option<Ball>) -> State {
  let State { game, .. } = state.clone();
  let mut board = game.board.clone();
  let selected_ball = game.selected_ball.clone();
  match maybe_ball {
    Some(ball) => {
      match selected_ball {
        Some(selected_ball) => {
          board[selected_ball.ball.place.row_index][selected_ball.ball.place.column_index] =
          Some(selected_ball.ball.clone());
          board[ball.place.row_index][ball.place.column_index] = None;
          let balls = get_balls(board.clone());
          let game = Game {
            board,
            selected_ball: Some(SelectedBall { 
              ball,
              is_selected_color: true
            }),
            balls,
            ..game
          };
          State { game, ..state }
        },
        None => {
          board[ball.place.row_index][ball.place.column_index] = None;
          let balls = get_balls(board.clone());
          let game = Game {
            board,
            selected_ball: Some(SelectedBall {
              ball,
              is_selected_color: true,
            }),
            balls,
            ..game
          };
          State { game, ..state }
        }
      }
    },
    None => {
      match selected_ball {
        Some(selected_ball) => {
          board[selected_ball.ball.place.row_index][selected_ball.ball.place.column_index] =
          Some(selected_ball.ball.clone());
          let balls = get_balls(board.clone());
          let game = Game {
            board,
            selected_ball: None,
            balls,
            ..game
          };
          State { game, ..state }
        },
        None => state.clone()
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
      height: 50,
    },
  }
}

fn get_radius(cell_width: i32) -> i32 {
  cell_width / 2 / 5 * 4
}

fn update_balls(game: Game) -> Game {
  let mut board = game.board.clone();
  for row_index in 0..board.len() {
    for column_index in 0..board[row_index].len() {
      let maybe_ball = board[row_index][column_index].clone();
      match maybe_ball {
        Some(ball) => {
          board[row_index][column_index] = Some(Ball {
            radius: get_radius(game.cell_width),
            position: get_position_for_ball(
              game.clone(),
              Place {
                row_index,
                column_index,
              },
            ),
            ..ball
          });
        }
        None => {}
      }
    }
  }
  let selected_ball = match game.clone().selected_ball {
    Some(selected_ball) => Some(SelectedBall {
      ball: Ball {
        radius: get_radius(game.cell_width),
        position: get_position_for_ball(game.clone(), selected_ball.ball.clone().place),
        ..selected_ball.ball
      },
      ..selected_ball
    }),
    None => None,
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
  let game = Game {
    board_width,
    line_width,
    cell_width,
    board_x,
    board_y,
    ..game
  };
  let game = update_balls(game);
  let balls = get_balls(game.board.clone());
  Game { balls, ..game }
}

fn add_balls(state: State) -> Game {
  let State { game, .. } = state;
  let maybe_find_place = find_place_for_ball(game.board.clone());
  match maybe_find_place {
    Some(place) => {
      let mut board = game.board.clone();
      let num = gen_ball_number();
      let ball = Ball {
        num,
        radius: get_radius(game.cell_width),
        place: place.clone(),
        position: get_position_for_ball(game.clone(), place.clone()),
      };
      let Place {
        row_index,
        column_index,
      } = place;
      board[row_index][column_index] = Some(ball);
      let balls = get_balls(board.clone());
      Game {
        board,
        balls,
        ..game
      }
    }
    None => Game {
      is_game_over: true,
      ..game
    },
  }
}
