mod layout;

use self::layout::AppLayout;
use crate::windows::common::{base_config, frameless};
use lampadion_assets::styles::WINDOW_INPUT_HINT;

use dioxus::desktop::{Config, LogicalSize};
use dioxus::prelude::*;

pub fn config() -> Config {
    let size = LogicalSize::new(400.0, 200.0);
    base_config(
        frameless("Lampadion — Input hint", size).with_min_inner_size(size),
    )
}

#[component]
pub fn Root() -> Element {
	rsx! {
		document::Stylesheet { href: WINDOW_INPUT_HINT }
		AppLayout {}
	}
}
