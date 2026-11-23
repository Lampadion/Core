mod state;
mod layout;
mod views;
mod components;
mod window;
mod tabs_builtin;

use self::layout::AppLayout;
use self::window::use_window_context;
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
	use_window_context();

	rsx! {
		document::Stylesheet { href: WINDOW_MAIN }
		AppLayout {}
	}
}
