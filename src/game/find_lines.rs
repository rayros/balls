use crate::game::find_path::board_static_to_vec;
use crate::game::state::Board;
use crate::game::state::Ball;
use serde::Serialize;

// 5 balls => 10 points
// +1 ball => +4 points
#[derive(Default, Serialize, Clone)]
pub struct Line {
  pub balls: Vec<Ball>,
  pub points: u32,
}

js_serializable!(Line);

pub fn find_lines(board: &Board) -> Vec<Line> {
  let board = board_static_to_vec(board);
  let diagonal_left = check_diagonal_left(&board);
  let diagonal_right = check_diagonal_right(&board);
  let columns = check_columns(&board);
  let rows = check_rows(&board);
  // console!(log, "find_lines");
  // console!(log, board.clone());
  [
    &rows[..],
    &columns[..],
    &diagonal_left[..],
    &diagonal_right[..],
  ]
  .concat()
}

fn get_points(balls_count: u32) -> u32 {
  10 + (balls_count - 5) * 4
}

fn create_line(balls: Vec<Ball>) -> Line {
  let balls_count = balls.len() as u32;
  Line {
    balls: balls,
    points: get_points(balls_count),
  }
}

// A B C D
// E F G H
// I J K L

// A B C D # #
// # E F G H #
// # # I J K L

fn check_diagonal_right(board: &Vec<Vec<Option<Ball>>>) -> Vec<Line> {
  let mut diagonal: Vec<Vec<Option<Ball>>> = vec![];
  for row_index in 0..board.len() {
    let mut column: Vec<Option<Ball>> = vec![];
    for _i in 0..row_index {
      column.push(None);
    }
    column.extend(board[row_index].iter().cloned());
    for _i in 0..board[0].len() - row_index {
      column.push(None);
    }
    diagonal.push(column);
  }
  check_columns(&diagonal)
}

// A B C D
// E F G H
// I J K L

// # # A B C D
// # E F G H #
// I J K L # #

fn check_diagonal_left(board: &Vec<Vec<Option<Ball>>>) -> Vec<Line> {
  let mut diagonal: Vec<Vec<Option<Ball>>> = vec![];
  for row_index in 0..board.len() {
    let mut column: Vec<Option<Ball>> = vec![];
    for _i in 0..board[0].len() - row_index - 1 {
      column.push(None);
    }
    column.extend(board[row_index].iter().cloned());
    for _i in 0..row_index {
      column.push(None);
    }
    diagonal.push(column);
  }
  check_columns(&diagonal)
}

fn check_columns(board: &Vec<Vec<Option<Ball>>>) -> Vec<Line> {
  let mut lines: Vec<Line> = vec![];
  if board.len() > 0 {
    for column_index in 0..board[0].len() {
      let column_lines = check_column(&board, column_index);
      lines.extend(column_lines.iter().cloned());
    }
  }
  lines
}

fn check_column(board: &Vec<Vec<Option<Ball>>>, column_index: usize) -> Vec<Line> {
  let mut check_lines = CheckLines {
    lines: vec![],
    balls: vec![],
  };
  for row in board.iter() {
    check_lines = check_cell(check_lines, &row[column_index]);
    // if column_index == 8 {
    //   console!(log, "8");
    //   console!(log, check_lines.clone());
    // }
  }
  check_lines.lines
}

fn check_rows(board: &Vec<Vec<Option<Ball>>>) -> Vec<Line> {
  let mut lines: Vec<Line> = vec![];
  for row_index in 0..board.len() {
    let row_lines = check_row(&board, row_index);
    lines.extend(row_lines.iter().cloned());
  }
  lines
}

fn check_row(board: &Vec<Vec<Option<Ball>>>, row_index: usize) -> Vec<Line> {
  let mut check_lines = CheckLines {
    lines: vec![],
    balls: vec![],
  };
  for column in board[row_index].iter() {
    check_lines = check_cell(check_lines, column);
    // if row_index == 0 {
    //   console!(log, check_lines.clone());
    // }
  }
  check_lines.lines
}

#[derive(Default, Serialize, Clone)]
struct CheckLines {
  lines: Vec<Line>,
  balls: Vec<Ball>,
}

js_serializable!(CheckLines);

fn check_cell(check_lines: CheckLines, cell: &Option<Ball>) -> CheckLines {
  let CheckLines {
    mut balls,
    mut lines,
  } = check_lines;
  match cell {
    Some(ball) => {
      let balls_count = balls.len();
      if balls_count == 0 {
        balls = vec![ball.clone()];
      } else {
        if balls[0].num == ball.num {
          balls.push(ball.clone());
          if balls.len() >= 5 {
            lines.push(create_line(balls.clone()));
            balls = vec![];
          }
        } else {
          if balls_count >= 5 {
            lines.push(create_line(balls.clone()));
          }
          balls = vec![ball.clone()];
        }
      }
    }
    None => {
      let balls_count = balls.len();
      if balls_count >= 5 {
        lines.push(create_line(balls.clone()));
      }
      balls = vec![];
    }
  }
  CheckLines { balls, lines }
}