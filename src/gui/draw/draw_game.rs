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
  fn draw_new_game_button(&self, state: &State);
  fn draw_privacy_policy_button(&self, state: &State);
}

impl DrawGameCtx for CanvasRenderingContext2d {
  fn draw_background(&self, width: i32, height: i32) {
    self.set_fill_style_color("#121212");
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
    self.set_fill_style_color("#2c2c2c");
    fill_rect(self, board_x, board_y, board_width, board_width);
    self.set_fill_style_color("#121212");
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
    self.set_fill_style_color("#1e1e1e");
    fill_rect(&self,
      state.game.board_x,
      state.game.board_y - state.game.navigation_height + 10,
      state.game.board_width, state.game.navigation_height - 20);
    self.set_fill_style_color("#9d9d9d");
    self.set_font("20px Roboto");
    let points_text = "SCORE:";
    let font_y = f64::from(state.game.board_y - state.game.navigation_height + 47);
    self.fill_text(
      points_text,
      f64::from(state.game.board_x + 10),
      font_y,
      None
    );
    self.set_fill_style_color("#e1e1e1");
    let text = format!("{}", state.game.points);
    self.fill_text(
      text.as_str(),
      f64::from(state.game.board_x + 15) + self.measure_text(&points_text).unwrap().get_width(),
      font_y,
      None
    );
  }
  
  fn draw_new_game_button(&self, state: &State) {
    self.set_font("20px Roboto");
    self.set_fill_style_color(color_map(3, None));
    let button = state.game.new_game_button.clone();
    fill_rect(&self,
      button.x,
      button.y,
      button.width,
      button.height
    );
    self.set_fill_style_color("#e1e1e1");
    let font_y = f64::from(button.y + button.height / 2 + 7);
    self.fill_text(
      button.text.as_str(),
      f64::from(button.x),
      font_y,
      None
    );
  }

  fn draw_privacy_policy_button(&self, state: &State) {
    self.set_font("10px Roboto");
    // self.set_fill_style_color(color_map(3, None));
    // let button = state.game.privacy_policy_link_button.button.clone();
    // fill_rect(&self,
    //   button.x,
    //   button.y,
    //   button.width,
    //   button.height
    // );
    let link_button = state.game.privacy_policy_link_button.clone().unwrap();
    self.set_fill_style_color("#e1e1e1");
    let font_y = f64::from(link_button.button.y + link_button.button.height / 2 + 5);
    self.fill_text(
      link_button.button.text.as_str(),
      f64::from(link_button.button.x),
      font_y,
      None
    );
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
  ctx.draw_points(&state);
  ctx.draw_new_game_button(&state);
  if let Some(_) = state.game.privacy_policy_link_button {
    ctx.draw_privacy_policy_button(&state);
  }
}
