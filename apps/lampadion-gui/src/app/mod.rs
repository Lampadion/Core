pub mod layout;
pub mod state;

use dioxus::prelude::*;
use layout::AppLayout;

const FAVICON: Asset = asset!("/assets/images/favicon.ico");
const MAIN_SCSS: Asset = asset!("/assets/styles/main.scss");

#[component]
pub fn App() -> Element {
    rsx! {
			document::Link { rel: "icon", href: FAVICON }
			document::Stylesheet { href: MAIN_SCSS }
			AppLayout {}
		}
}
