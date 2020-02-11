use crate::canvas::watch_click_event;
use crate::gui;
use crate::store::{Action, State, Store, View};
use crate::throttle::Throttle;
use std::cell::RefCell;
use std::rc::Rc;

pub struct _Story {
  pub store: Store,
  story_rc: Option<Story>,
}

impl _Story {
  pub fn new(store: Store) -> _Story {
    _Story {
      store,
      story_rc: None,
    }
  }

  pub fn story(&self, action: Action) {
    self.store.borrow_mut().dispatch(action.clone());
    let store = self.store.clone();
    let story_rc = self.story_rc.clone().unwrap();
    let draw_story_rc = self.story_rc.clone().unwrap();
    let a = move || {
      let store = draw_story_rc.clone().borrow().store.clone();
      gui::draw(store.borrow().state.clone());
    };
    let draw_throttle = Throttle::new(a, 1000 / 60);
    match action {
      Action::None => {
        gui::load_fonts(story_rc);
      }
      Action::FontLoaded => {
        let (canvas, width, height) = gui::create_canvas("#canvas");
        self.story(Action::NewCanvas {
          canvas,
          width,
          height,
        });
      }
      Action::NewCanvas { canvas, .. } => {
        self.story(Action::ChangeView { view: View::Game });
        watch_click_event(story_rc, canvas);
      }
      Action::Draw => {
        draw_throttle.update();
      }
      Action::WindowResize => {
        let state: State = store.borrow().state.clone();
        let canvas = state.canvas.unwrap();
        let (width, height) = gui::resize_canvas_to_window_size(&canvas.element);
        self.story(Action::CanvasResize { width, height });
      }
      Action::CanvasResize { .. } => {
        self.story(Action::Draw);
      }
      Action::Click { x, y } => {
        let state: State = store.borrow().state.clone();
        match state.view {
          View::None => {}
          View::Menu => {
            if state.menu.start_button.intersect(x, y) {
              self.story(Action::ChangeView { view: View::Game });
            }
          }
          View::Game => {}
        }
      }
      Action::ChangeView { view } => {
        match view {
          View::Game => {
            let state: State = store.borrow().state.clone();
            if state.game.balls.len() == 0 {
              self.story(Action::AddBalls);
            }
          }
          _ => {}
        }
        gui::draw(store.borrow().state.clone());
      }
      Action::AddBalls => {
        console!(log, "balls added");
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
  store
}
