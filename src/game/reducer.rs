use crate::game::state::Animation;
use crate::game::state::Step;
use crate::game::find_path::Path;
use crate::game::state::Config;
use super::selectors::*;
use crate::game::action::Action;
use crate::game::find_lines::find_lines;
use crate::game::state::Ball;
use crate::game::state::Board;
use crate::game::state::Button;
use crate::game::state::Game;
use crate::game::state::LinkButton;
use crate::game::state::Place;
use crate::game::state::SelectedBall;
use crate::game::state::State;

pub fn reducer(state: &State, action: &Action) -> State {
  match action {
    Action::None => state.clone(),
    Action::ConfigLoaded { config } => config_loaded(config, state),
    Action::FontLoaded => state.clone(),
    Action::Draw => state.clone(),
    Action::WindowResize => state.clone(),
    Action::Click { .. } => state.clone(),
    Action::CanvasResize { width, height } => State {
      canvas_width: *width,
      canvas_height: *height,
      game: resize_game(state.clone()),
      ..state.clone()
    },
    Action::NewCanvas {
      canvas,
      width,
      height,
    } => {
      let state = State {
        canvas: Some(canvas.clone()),
        canvas_width: *width,
        canvas_height: *height,
        ..state.clone()
      };
      State {
        game: resize_game(state.clone()),
        ..state
      }
    },
    Action::AddBalls => add_balls(state.clone(), 3),
    Action::SelectBall { maybe_ball } => select_ball(state.clone(), maybe_ball.clone()),
    Action::ChangeSelectedBallColor { ball } => match state.game.selected_ball.clone() {
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
      }
      None => state.clone(),
    },
    Action::MoveBall { path } => animation_start(path, state),
    Action::CheckLines {} => check_lines(state),
    Action::NewGame => new_game(state),
    Action::Animate => animation_step(state)
  }
}

fn move_ball(path: &Path, state: &State) -> State {
  let place = path.last().unwrap();
  let game = state.game.clone();
  let selected_ball = state.game.selected_ball.clone().unwrap();
  let selected_ball = SelectedBall {
    ball: Ball {
      place: place.clone(),
      position: get_position_for_ball(&game, &place),
      ..selected_ball.ball
    },
    is_selected_color: true,
  };
  let state = add_selected_ball_to_board(&state, selected_ball);
  state
}

fn animation_start(path: &Path, state: &State) -> State {
  let selected_ball = state.game.selected_ball.clone().unwrap();
  let mut animation_steps: Vec<Step> = vec![];
  for path_place in path {
    animation_steps.push(Step {
      ball: Ball {
        place: path_place.clone(),
        position: get_position_for_ball(&state.game, &path_place),
        ..selected_ball.ball
      }
    });
  }
  let state = state.clone();
  State {
    game: Game {
      selected_ball: None,
      animation: Some(Animation {
        steps: animation_steps,
        current_step: 0
      }),
      ..state.game
    },
    ..state
  }
}

fn animation_step(state: &State) -> State {
  let animation: Animation = state.game.animation.clone().unwrap();
  if animation.current_step == animation.steps.len() - 1 {
    return animation_end(state);
  }
  State {
    game: Game {
      animation: Some(Animation {
        current_step: animation.current_step + 1,
        ..animation
      }),
      ..state.game.clone()
    },
    ..state.clone()
  }
}

fn animation_end(state: &State) -> State {
  let animation = state.game.animation.clone().unwrap();
  let last_animation_step = animation.steps.last().unwrap();
  let ball = last_animation_step.ball.clone();
  let selected_ball = SelectedBall {
    ball,
    is_selected_color: true,
  };
  let state = add_selected_ball_to_board(&state, selected_ball);
  State {
    game: Game {
      animation: None,
      ..state.game
    },
    ..state
  }
}

fn config_loaded(config: &Config, state: &State) -> State {
  State {
    config: config.clone(),
    ..state.clone()
  }
}

fn new_game(state: &State) -> State {
  let board: Board = Default::default();
  let selected_ball: Option<SelectedBall> = None;
  let balls: Vec<Ball> = vec![];
  State {
    game: Game {
      board,
      selected_ball,
      balls,
      ..state.game.clone()
    },
    ..state.clone()
  }
}

fn check_lines(state: &State) -> State {
  let mut board = state.game.board.clone();
  let mut points = state.game.points;
  let lines = find_lines(&state.game.board);
  for line in lines.clone() {
    points += line.points;
    for ball in line.balls {
      board[ball.place.row_index][ball.place.column_index] = None;
    }
  }
  // console!(log, points.clone());
  let balls = get_balls(&board);
  State {
    game: Game {
      board,
      balls,
      points,
      ..state.game.clone()
    },
    ..state.clone()
  }
}

fn add_selected_ball_to_board(state: &State, selected_ball: SelectedBall) -> State {
  let game = state.game.clone();
  let mut board = game.board;
  board[selected_ball.ball.place.row_index][selected_ball.ball.place.column_index] =
    Some(selected_ball.ball.clone());
  let balls = get_balls(&board);
  let game = Game {
    board,
    selected_ball: None,
    balls,
    ..game
  };
  State {
    game,
    ..state.clone()
  }
}

fn select_ball(state: State, maybe_ball: Option<Ball>) -> State {
  let State { game, .. } = state.clone();
  let mut board = game.board.clone();
  let selected_ball = game.selected_ball.clone();
  match maybe_ball {
    Some(ball) => match selected_ball {
      Some(selected_ball) => {
        board[selected_ball.ball.place.row_index][selected_ball.ball.place.column_index] =
          Some(selected_ball.ball.clone());
        board[ball.place.row_index][ball.place.column_index] = None;
        let balls = get_balls(&board);
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
      None => {
        board[ball.place.row_index][ball.place.column_index] = None;
        let balls = get_balls(&board);
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
    },
    None => match selected_ball {
      Some(selected_ball) => add_selected_ball_to_board(&state, selected_ball),
      None => state.clone(),
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
              &game,
              &Place {
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
        position: get_position_for_ball(&game, &selected_ball.ball.place),
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
    canvas,
    ..
  } = state;
  let navigation_height = 80;
  let privacy_policy_height = 30;
  let max_board_height = canvas_height - navigation_height - privacy_policy_height;
  let board_width = if max_board_height < canvas_width {
    max_board_height
  } else {
    canvas_width
  };
  let board_x = (canvas_width - board_width) / 2;
  let board_y = navigation_height
    + (canvas_height - navigation_height - privacy_policy_height - board_width) / 2;
  let line_width = board_width / 100;
  let cell_width = board_width / 9;
  let ctx = canvas.unwrap().ctx;
  let new_game_text = "  - NEW GAME -   ";
  ctx.set_font("20px Roboto");
  let width = ctx.measure_text(new_game_text).unwrap().get_width() as i32;
  let new_game_button = Button {
    text: String::from(new_game_text),
    x: board_x + board_width - width - 20,
    y: board_y - navigation_height + 20,
    width,
    height: navigation_height - 40,
  };
  let privacy_policy_link_button = if state.config.no_privacy_policy {
    None
  } else {
    let privacy_policy_text = "  PRIVACY POLICY   ";
    ctx.set_font("10px Roboto");
    let width = ctx.measure_text(privacy_policy_text).unwrap().get_width() as i32;
    let privacy_policy_button = Button {
      text: String::from(privacy_policy_text),
      x: board_x + board_width / 2 - width / 2,
      y: board_y + board_width,
      width,
      height: privacy_policy_height,
    };
    Some(LinkButton {
      link: get_privacy_policy_link(),
      button: privacy_policy_button,
    })
  };
  let game = Game {
    board_width,
    line_width,
    cell_width,
    board_x,
    board_y,
    navigation_height,
    new_game_button,
    privacy_policy_link_button,
    ..game
  };
  let game = update_balls(game);
  let balls = get_balls(&game.board);
  Game { balls, ..game }
}

fn add_balls(state: State, num: usize) -> State {
  let mut state = state;
  for _x in 0..num {
    state = add_ball(state);
  }
  state
}

fn add_ball(state: State) -> State {
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
        position: get_position_for_ball(&game, &place),
      };
      let Place {
        row_index,
        column_index,
      } = place;
      board[row_index][column_index] = Some(ball);
      let balls = get_balls(&board);
      State {
        game: Game {
          board,
          balls,
          ..game
        },
        ..state
      }
    }
    None => {
      console!(log, "No place for new ball");
      State {
        game: Game {
          is_game_over: true,
          ..game
        },
        ..state
      }
    }
  }
}
