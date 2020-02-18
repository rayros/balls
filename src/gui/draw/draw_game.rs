use crate::game::{Game, Ball, State, SelectedBall};
use super::fill_rect::fill_rect;
use stdweb::web::{CanvasRenderingContext2d, FillRule};
use std::f64::consts::PI;

fn color_map(num: u8, selected: Option<bool>) -> &'static str {
  let selected = selected.unwrap_or(false);
  let color = match num {
    1 => ("#f44336", "#b71c1c"),
    2 => ("#9c27b0", "#4a148c"),
    3 => ("#3f51b5", "#1a237e"),
    4 => ("#03a9f4", "#01579b"),
    5 => ("#009688", "#004d40"),
    6 => ("#8bc34a", "#33691e"),
    7 => ("#ffeb3b", "#f57f17"),
    8 => ("#f57f17", "#e65100"),
    9 => ("#795548", "#3e2723"),
    _ => ("black", "black")
  };
  if selected { color.1 } else { color.0 }
}

trait DrawGameCtx {
  fn draw_background(&self, width: i32, height: i32);
  fn draw_board(&self, state: &State);
  fn draw_ball(&self, ball: Ball);
  fn draw_selected_ball(&self, selected_ball: SelectedBall);
  fn draw_points(&self, state: &State);
}

impl DrawGameCtx for CanvasRenderingContext2d {
  fn draw_background(&self, width: i32, height: i32) {
    self.set_fill_style_color("#606368");
    fill_rect(self, 0, 0, width, height);
  }

  fn draw_ball(&self, ball: Ball) {
    self.begin_path();
    self.arc(f64::from(ball.position.0), f64::from(ball.position.1), f64::from(ball.radius), 0.0, 2.0 * PI, false);
    self.set_fill_style_color(color_map(ball.num, None));
    self.fill(FillRule::NonZero);
    self.close_path();
  }

  fn draw_selected_ball(&self, selected_ball: SelectedBall) {
    let stroke_width = selected_ball.ball.radius / 10;
    self.set_stroke_style_color("black");
    self.set_line_width(stroke_width as f64);
    self.begin_path();
    self.arc(f64::from(selected_ball.ball.position.0), f64::from(selected_ball.ball.position.1), f64::from(selected_ball.ball.radius - stroke_width / 2), 0.0, 2.0 * PI, false);
    if selected_ball.is_selected_color == true {
      self.set_fill_style_color(color_map(selected_ball.ball.num, Some(true)));
    } else {
      self.set_fill_style_color(color_map(selected_ball.ball.num, None));
    }
    self.fill(FillRule::NonZero);
    self.close_path();
  }

  fn draw_board(&self, state: &State) {
    let State { 
      game: Game {
        board_x,
        board_y,
        board_width,
        line_width,
        cell_width,
        balls,
        selected_ball,
        ..
      },
      ..
    } = state.clone();
    self.set_fill_style_color("#3c4043");
    fill_rect(self, board_x, board_y, board_width, board_width);
    self.set_fill_style_color("#afb2b7");
    for i in 1..9 {
      fill_rect(self, board_x - (line_width / 2) + (cell_width * i), board_y, line_width, board_width);
    }
    for i in 1..9 {
      fill_rect(self, board_x, board_y - (line_width / 2) + (cell_width * i), board_width, line_width);
    }
    for ball in balls.into_iter() {
      if ball.num != 0 {
        self.draw_ball(ball);
      }
    }
    if let Some(selected_ball) = selected_ball {
      self.draw_selected_ball(selected_ball);
    }
  }

  fn draw_points(&self, state: &State) {

  }
}

pub fn draw_game(state: State) {
  let State {
    canvas,
    canvas_height,
    canvas_width,
    ..
  } = state.clone();
  let canvas = canvas.unwrap();
  let ctx = canvas.ctx;
  ctx.draw_background(canvas_width, canvas_height);
  ctx.draw_board(&state);
}
