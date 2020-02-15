use std::rc::Rc;
use std::cell::RefCell;

pub struct _Store<S, A> {
  pub state: S,
  reducer: fn(state: &S, action: &A) -> S,
}

impl<S, A> _Store<S, A> {
  pub fn new(
    reducer: fn(state: &S, action: &A) -> S,
    state: S
  ) -> _Store<S, A> {
    _Store {
      state,
      reducer
    }
  }

  pub fn dispatch(&mut self, action: A) {
    self.state = (self.reducer)(&self.state, &action);
  }
}

pub type Store<S, A> = Rc<RefCell<_Store<S, A>>>;

pub fn get_store<S: Default, A>(reducer: fn(state: &S, action: &A) -> S) -> Store<S, A> {
  Rc::new(RefCell::new(_Store::new(reducer, S::default())))
}
