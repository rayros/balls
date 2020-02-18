use std::fmt;

#[derive(Debug, Clone)]
pub enum View {
  None,
  Game,
}

impl Default for View {
  fn default() -> Self {
    View::None
  }
}

impl fmt::Display for View {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({:?})", self)
  }
}