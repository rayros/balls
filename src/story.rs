use crate::canvas::watch_click_event;
use crate::store::{Store, Action, State};
use crate::gui;
use std::rc::Rc;
use std::cell::RefCell;

pub struct _Story {
  pub store: Store,
  story_rc: Option<Story>
}

impl _Story {
  pub fn new(
    store: Store
  ) -> _Story {
    return _Story {
      store,
      story_rc: None
    };
  }

  pub fn story(&self, action: Action) {
    self.store.borrow_mut().dispatch(action.clone());
    let store = self.store.clone();
    let story_rc = self.story_rc.clone().unwrap();
    match action {
      Action::None => {
        gui::load_fonts(story_rc.clone());
      },
      Action::FontLoaded => {
        let (canvas, width, height) = gui::create_canvas("#canvas");
        self.story(Action::NewCanvas { canvas, width, height });
      },
      Action::NewCanvas { canvas, height: _, width: _ } => {
        watch_click_event(story_rc.clone(), canvas);
      },
      Action::Draw => {
        let state: State = store.borrow().state.clone();
        gui::draw(state);
      },
      Action::WindowResize => {
        let state: State = store.borrow().state.clone();
        let canvas = state.canvas.unwrap();
        let (width, height) = gui::resize_canvas_to_window_size(&canvas.element);
        self.story(Action::CanvasResize { width, height });
      },
      Action::CanvasResize { width: _, height: _ } => {},
      Action::Click => {
        console!(log, "click");
      }
    }
  }

  pub fn add_rc(&mut self, story: Story) {
    self.story_rc = Some(story);
  }
}


pub type Story = Rc<RefCell<_Story>>;

pub fn get_story(store: Store) -> Story {
  let store = Rc::new(RefCell::new(_Story::new(store)));
  store.borrow_mut().add_rc(store.clone());
  return store;
}
