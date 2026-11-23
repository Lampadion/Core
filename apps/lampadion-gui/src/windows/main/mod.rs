mod state;
mod layout;
mod views;
mod components;

use self::layout::AppLayout;
use crate::windows::common::{base_config, frameless};
use lampadion_assets::styles::WINDOW_MAIN;

use dioxus::desktop::{Config, LogicalSize};
use dioxus::prelude::*;

pub fn config() -> Config {
    let size = LogicalSize::new(1280.0, 900.0);
    base_config(
        frameless("Lampadion", size).with_min_inner_size(size),
    )
}

#[component]
pub fn Root() -> Element {
	rsx! {
		// document::Stylesheet { href: window_scss!("main") }
		document::Stylesheet { href: WINDOW_MAIN }
		AppLayout {}
	}
}
