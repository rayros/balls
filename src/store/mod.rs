mod reducer;
mod store;
mod action;
mod state;
mod view;
mod selectors;
pub use view::View;
pub use action::Action;
pub use state::{State, Game, Ball};
use reducer::reducer;

use std::rc::Rc;
use std::cell::RefCell;

pub type Store = Rc<RefCell<store::Store<State, Action>>>;

pub fn get_store() -> Store {
  Rc::new(RefCell::new(store::Store::new(reducer, State::default())))
}
