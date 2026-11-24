mod preview;
mod layout;
mod components;
mod hooks;

use dioxus::prelude::*;
use crate::website::layout::WebsiteLayout;

use lampadion_assets::icons::FAVICON;
use lampadion_assets::styles::{RESET, WEBSITE_MAIN};

#[component]
pub fn App() -> Element {
	rsx! {
		document::Link { rel: "icon", href: FAVICON, r#type: "image/svg+xml" }
		document::Stylesheet { href: RESET }
		document::Stylesheet { href: WEBSITE_MAIN }

		WebsiteLayout {}

	}
}
