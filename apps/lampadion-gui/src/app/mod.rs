mod layout;
mod state;

use dioxus::prelude::*;
use layout::AppLayout;

const FAVICON: Asset = asset!("/assets/images/favicon.ico");
const MAIN_SCSS: Asset = asset!(
    "/assets/styles/main.scss",
    AssetOptions::css().with_minify(false)
);

const RESET_SCSS: Asset = asset!(
		"/assets/styles/utils/_reset.scss",
		AssetOptions::css().with_minify(false)
);

#[component]
pub fn App() -> Element {
    rsx! {
			document::Link { rel: "icon", href: FAVICON }
			document::Link { rel: "stylesheet", href: MAIN_SCSS }
			document::Link { rel: "stylesheet", href: RESET_SCSS }
			AppLayout {}
		}
}
