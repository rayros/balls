fn get_admob_unit_id() -> String {
  String::from("ca-app-pub-7811559275890870/3853251737")
}

pub fn show_ad() {
  js! {
    console.log("Ads show.");
    if (!window.admob) return;
    admob.interstitial.show();
  }
}

pub fn load_ad() {
  let unit_ad = get_admob_unit_id();
  js! {
    console.log("Ad load.");
    if (!window.admob) return;
    var config = {
      id: {
        android: @{unit_ad}
      }
    };
    console.log(config);
    admob.interstitial.load(config);
  }
}