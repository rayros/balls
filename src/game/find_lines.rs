use crate::game::find_path::board_static_to_vec;
use crate::game::state::Ball;
use crate::game::state::Board;
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
  if balls_count < 5 {
    console!(log, "Wrong", balls_count);
  }
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

fn check_columns(board: &[Vec<Option<Ball>>]) -> Vec<Line> {
  let mut lines: Vec<Line> = vec![];
  if !board.is_empty() {
    for column_index in 0..board[0].len() {
      let column_lines = check_column(&board, column_index);
      lines.extend(column_lines.iter().cloned());
    }
  }
  lines
}

#[cfg(test)]
mod tests {
  use crate::game::state::Place;
  use super::*;
  use std::panic;

  fn board_mock_to_vec(board: Vec<Vec<&str>>) -> Vec<Vec<Option<Ball>>> {
    let mut board_2d_vec: Vec<Vec<Option<Ball>>> = vec![];
    for row_item in board.iter() {
      let mut row_item_vec = vec![];
      for item in row_item.iter().cloned() {
        if item == "" {
          row_item_vec.push(None);
        } else {
          row_item_vec.push(Some(Ball {
            num: item.parse().unwrap(),
            place: Place {
              row_index: 0,
              column_index: 0,
            },
            position: (0, 0),
            radius: 0,
          }))
        }
      }
      board_2d_vec.push(row_item_vec);
    }
    board_2d_vec
  }

  #[test]
  fn test_check_columns() {
    let board = board_mock_to_vec(vec![
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", "1"],
    ]);
    let lines = check_columns(&board);
    let points = lines[0].points;
    console!(log, "");
    assert_eq!(points, 14, "Expected {} to be {}", points, 14);

    let board = board_mock_to_vec(vec![
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", "1"],
      vec!["", "", "", "", "", ""],
    ]);
    let lines = check_columns(&board);
    let points = lines[0].points;
    console!(log, "");
    assert_eq!(points, 10, "Expected {} to be {}", points, 10);

    let board = board_mock_to_vec(vec![
      vec!["", "", "", "", "2", "1"],
      vec!["", "", "", "", "2", "1"],
      vec!["", "", "", "", "2", "1"],
      vec!["", "", "", "", "2", "1"],
      vec!["", "", "", "", "2", "1"],
      vec!["", "", "", "", "", "1"],
    ]);
    let lines = check_columns(&board);
    console!(log, "");
    assert_eq!(lines[0].points, 10, "Expected {} to be {}", points, 10);
    assert_eq!(lines[1].points, 14, "Expected {} to be {}", points, 14);

  }

  #[test]
  fn test_check_rows() {
    let board = board_mock_to_vec(vec![
      vec!["1", "1", "1", "1", "1", "1"],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
    ]);
    let lines = check_rows(&board);
    let points = lines[0].points;
    console!(log, "");
    assert_eq!(points, 14, "Expected {} to be {}", points, 14);

    let board = board_mock_to_vec(vec![
      vec!["", "1", "1", "1", "1", "1"],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
    ]);
    let lines = check_rows(&board);
    let points = lines[0].points;
    console!(log, "");
    assert_eq!(points, 10, "Expected {} to be {}", points, 10);

    let board = board_mock_to_vec(vec![
      vec!["", "", "", "", "", ""],
      vec!["", "2", "2", "2", "2", "2"],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
      vec!["1", "1", "1", "1", "1", "1"],
      vec!["", "", "", "", "", ""],
    ]);
    let lines = check_rows(&board);
    console!(log, "");
    assert_eq!(lines[0].points, 10, "Expected {} to be {}", points, 10);
    assert_eq!(lines[1].points, 14, "Expected {} to be {}", points, 14);

    let board = board_mock_to_vec(vec![
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "2", "2"],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
      vec!["", "", "", "", "", ""],
    ]);
    let lines = check_rows(&board);
    console!(log, "");
    assert_eq!(lines.len(), 0);

  }

}

fn check_column(board: &[Vec<Option<Ball>>], column_index: usize) -> Vec<Line> {
  let mut check_lines = CheckLines {
    lines: vec![],
    balls: vec![],
  };
  for (row_index, row) in board.iter().enumerate() {
    check_lines = check_cell(
      check_lines,
      &row[column_index],
      row_index == board.len() - 1,
    );
    // if column_index == 8 {
    //   console!(log, "8");
    //   console!(log, check_lines.clone());
    // }
  }
  check_lines.lines
}

fn check_rows(board: &[Vec<Option<Ball>>]) -> Vec<Line> {
  let mut lines: Vec<Line> = vec![];
  for row_index in 0..board.len() {
    let row_lines = check_row(&board, row_index);
    lines.extend(row_lines.iter().cloned());
  }
  lines
}

fn check_row(board: &[Vec<Option<Ball>>], row_index: usize) -> Vec<Line> {
  let mut check_lines = CheckLines {
    lines: vec![],
    balls: vec![],
  };
  for (column_index, column) in board[row_index].iter().enumerate() {
    check_lines = check_cell(
      check_lines,
      column,
      column_index == board[row_index].len() - 1,
    );
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

fn check_cell(check_lines: CheckLines, cell: &Option<Ball>, last_one: bool) -> CheckLines {
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
          if last_one && balls.len() >= 5 {
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
