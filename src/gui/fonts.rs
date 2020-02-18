use crate::story::Story;
use crate::game::Action;
use std::collections::HashMap;
use stdweb::{Object};


pub fn load(family: &str, source: &str, descriptors: Option<HashMap<&str, &str>>, story: Story) {
  let _descriptors = descriptors.unwrap_or_default();
  let descriptors_object = Object::from(_descriptors);
  let c = move || {
    story.borrow().story(Action::FontLoaded);
  };
  js! {
    var action_font_loaded = @{c};
    // console.log(@{descriptors_object.clone()});
    // console.log(@{source});
    const font = new FontFace(@{family}, @{source}, @{descriptors_object});
    document.fonts.add(font);
    // console.log(font);
    return font.load().then(() => action_font_loaded());
  }
}
