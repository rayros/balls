use stdweb::Once;
use crate::game::state::Config;
use crate::story::Story;
use crate::game::Action;

pub fn load_config(story: Story) {
  let callback = move |config: Config| {
    story.borrow().story(Action::ConfigLoaded { config });
  };
  js! {
    var config_loaded = @{Once(callback)};
    fetch( "./assets/config.json" )
      .then((response) => response.json())
      .then(config_loaded);
  }
}