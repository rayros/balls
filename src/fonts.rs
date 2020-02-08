use std::collections::HashMap;
use stdweb::{Promise, Object};
use stdweb::unstable::TryInto;

pub fn load(family: &str, source: &str, descriptors: Option<HashMap<String, String>>) -> Option<Promise> {
  let _descriptors = descriptors.unwrap_or(HashMap::new());
  let descriptors_object = Object::from(_descriptors);
  return Promise::from_thenable(
    &js! {
      // console.log(@{descriptors_object.clone()});
      const font = new FontFace(@{family}, @{source}, @{descriptors_object});
      document.fonts.add(font);
      return font.load();
    }
    .try_into()
    .unwrap(),
  );
}
