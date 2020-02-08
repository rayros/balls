#[macro_use]
extern crate stdweb;
use stdweb::{Value};

mod fonts;

fn main() {
  stdweb::initialize();

  let _handle = fonts::load("Amatica SC", "local('Amatica SC'), url(https://fonts.gstatic.com/s/amaticasc/v1/r4IyjqPgTL10SERuDuUzpAzyDMXhdD8sAj6OAJTFsBI.woff2) format('woff2')", None)
    .unwrap()
    .done(|result: Result<Value, Value>| match result {
      Ok(value) => {
        console!(log, value);
      }
      Err(error) => {
        console!(error, error);
      }
    })
    .leak();

  stdweb::event_loop();
}
