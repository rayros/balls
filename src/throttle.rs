use stdweb::Value;

#[derive(Debug)]
pub struct Throttle(Value);

impl Throttle {
  pub fn new<F>(callback: F, wait: u32) -> Self
  where
    F: Fn() + 'static,
  {
    Throttle(js!(
        var callback = @{callback};
        var wait = @{wait};

        var state = {
          wait: wait,
          active: false,
          callback: callback,
        };

        return state;
    ))
  }

  pub fn update(&self) {
    js! { @(no_return)
      var state = @{&self.0};
      if (state.active === false) {
        state.active = true;
        setTimeout(() => {
          state.callback();
        }, 0);
        setTimeout(() => {
          state.callback();
          state.active = false;
        }, state.wait);
      }
    }
  }
}
