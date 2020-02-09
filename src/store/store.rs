pub struct Store<S, A> {
  pub state: S,
  reducer: fn(state: &S, action: &A) -> S,
}

impl<S, A> Store<S, A> {
  pub fn new(
    reducer: fn(state: &S, action: &A) -> S,
    state: S
  ) -> Store<S, A> {
    return Store {
      state,
      reducer
    };
  }

  pub fn dispatch(&mut self, action: A) {
    self.state = (self.reducer)(&self.state, &action);
  }
}