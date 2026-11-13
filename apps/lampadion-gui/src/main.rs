use dioxus::prelude::*;

mod app;
mod components;
mod views;

use dioxus::desktop::tao::window::WindowBuilder;

fn main() {
		LaunchBuilder::desktop()
		.with_cfg(
				dioxus::desktop::Config::new()
						.with_window(
								WindowBuilder::new()
										.with_decorations(false)
						)
		)
		.launch(app::App);
}
