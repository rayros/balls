use crate::store::{Game, Ball};
use super::fill_rect::fill_rect;
use crate::store::State;
use stdweb::web::{CanvasRenderingContext2d, FillRule};
use std::f64::consts::PI;

trait DrawGameCtx {
  fn draw_background(&self, width: i32, height: i32);
  fn draw_ball(&self, ball: Ball);
  fn draw_board(&self, game: Game);
}

fn color_map(num: u8) -> &'static str {
  match num {
    2 => "red",
    _ => "black"
  }
}

impl DrawGameCtx for CanvasRenderingContext2d {
  fn draw_background(&self, width: i32, height: i32) {
    self.set_fill_style_color("#606368");
    fill_rect(self, 0, 0, width, height);
  }

  fn draw_ball(&self, ball: Ball) {
    self.begin_path();
    self.arc(f64::from(ball.position.0), f64::from(ball.position.1), f64::from(ball.radius), 0.0, 2.0 * PI, false);
    self.set_fill_style_color(color_map(ball.num));
    self.fill(FillRule::NonZero);
  }

  fn draw_board(&self, game: Game) {
    let Game {
      board_x,
      board_y,
      board_width,
      line_width,
      cell_width,
      ..
    } = game;
    self.set_fill_style_color("#3c4043");
    fill_rect(self, board_x, board_y, board_width, board_width);
    self.set_fill_style_color("#afb2b7");
    for i in 1..9 {
      fill_rect(self, board_x - (line_width / 2) + (cell_width * i), board_y, line_width, board_width);
    }
    for i in 1..9 {
      fill_rect(self, board_x, board_y - (line_width / 2) + (cell_width * i), board_width, line_width);
    }
    for ball in game.balls.into_iter() {
      if ball.num != 0 {
        self.draw_ball(ball);
      }
    }
  }
}

pub fn draw_game(state: State) {
  let State {
    canvas_height,
    canvas_width,
    game,
    ..
  } = state;
  let canvas = state.canvas.unwrap();
  let ctx = canvas.ctx;
  ctx.draw_background(canvas_width, canvas_height);
  ctx.draw_board(game);
}
