use crate::store::Ball;
use crate::canvas::watch_click_event;
use crate::gui;
use crate::store::{Action, State, Store, View};
use crate::throttle::Throttle;
use std::cell::RefCell;
use std::rc::Rc;

fn maybe_ball_intersect(balls: Vec<Ball>, x: i32, y: i32) -> Option<Ball> {
  let maybe_ball = balls.into_iter().find(|ball| {
    ball.intersect(x, y)
  });
  maybe_ball
}

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
      let store = draw_story_rc.clone().borrow_mut().store.clone();
      // console!(log, "draw", store.borrow().state.game.clone());
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
          View::Game => {
            let maybe_click_ball = maybe_ball_intersect(state.game.balls, x, y);
            if let Some(ball) = maybe_click_ball {
              self.story(Action::SelectBall { ball });
            }
            if let Some(selected_ball) = state.game.selected_ball {
              if selected_ball.ball.intersect(x, y) {
                self.story(Action::SelectBall { ball: selected_ball.ball });
              }
            }
          }
        }
      }
      Action::ChangeView { view } => {
        match view {
          View::Game => {
            let state: State = store.borrow().state.clone();
            if state.game.balls.is_empty() {
              for _x in 0..10 {
                self.story(Action::AddBalls);
              }
              
            }
          }
          _ => {}
        }
        self.story(Action::Draw);
      }
      Action::AddBalls => {
        self.story(Action::Draw);
      }
      Action::SelectBall { .. } => {
        self.story(Action::Draw);
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
