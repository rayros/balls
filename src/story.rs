use crate::canvas::watch_click_event;
use crate::game::{Action, Ball, State, View, equal_place, maybe_place_intersect, find_path};
use crate::gui;
use crate::store::Store;
use crate::throttle::Throttle;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::web::set_timeout;

fn maybe_ball_intersect(balls: &Vec<Ball>, x: i32, y: i32) -> Option<&Ball> {
  let maybe_ball = balls.into_iter().find(|ball| ball.intersect(x, y));
  maybe_ball
}

pub struct _Story {
  pub store: Store<State, Action>,
  story_rc: Option<Story>,
}

impl _Story {
  pub fn new(store: Store<State, Action>) -> _Story {
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
            let maybe_click_ball = maybe_ball_intersect(&state.game.balls, x, y);
            match maybe_click_ball {
              Some(ball) => {
                self.story(Action::SelectBall {
                  maybe_ball: Some(ball.clone()),
                });
              }
              None => {
                let maybe_selected_ball = state.game.selected_ball.clone();
                match maybe_selected_ball {
                  Some(selected_ball) => {
                    let maybe_place_on_board = maybe_place_intersect(&state.game, x, y);
                    match maybe_place_on_board {
                      Some(place) => {
                        let path = find_path(&state.game.board, selected_ball.ball.place, place);
                        match path {
                          Some(path) => {
                            self.story(Action::MoveBall { path });
                          },
                          None => {}
                        }
                        
                      },
                      None => {
                        self.story(Action::SelectBall { maybe_ball: None });
                      }
                    }
                  },
                  None => {
                    self.story(Action::SelectBall { maybe_ball: None });
                  }
                }
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
              self.story(Action::AddBalls);
            }
          }
          _ => {}
        }
        self.story(Action::Draw);
      }
      Action::AddBalls => {
        self.story(Action::Draw);
      }
      Action::SelectBall { maybe_ball } => {
        if let Some(ball) = maybe_ball {
          self.story(Action::ChangeSelectedBallColor { ball });
        }
      }
      Action::ChangeSelectedBallColor { ball } => {
        self.story(Action::Draw);
        if let Some(selected_ball) = store.borrow().state.game.selected_ball.clone() {
          if equal_place(selected_ball.ball.place, ball.clone().place) {
            let story = self.story_rc.clone().unwrap();
            set_timeout(
              move || {
                story
                  .borrow()
                  .story(Action::ChangeSelectedBallColor { ball });
              },
              300,
            );
          }
        }
      },
      Action::MoveBall { .. } => {
        self.story(Action::Draw);
        self.story(Action::AddBalls);
        self.story(Action::CheckLines);
      },
      Action::CheckLines => {}
    }
  }

  pub fn add_rc(&mut self, story: Story) {
    self.story_rc = Some(story);
  }
}

pub type Story = Rc<RefCell<_Story>>;

pub fn get_story(store: Store<State, Action>) -> Story {
  let store = Rc::new(RefCell::new(_Story::new(store)));
  store.borrow_mut().add_rc(store.clone());
  store
}
