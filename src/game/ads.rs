fn get_admob_unit_id() -> String {
  String::from("ca-app-pub-7811559275890870/3853251737")
}

pub fn show_ad() {
  let unit_ad = get_admob_unit_id();
  js! {
    console.log("Ads show.");
    if (!window.admob) return;
    admob.interstitial.load({
      id: {
        android: @{unit_ad},
      },
    }).then(() => admob.interstitial.show())
  }
}