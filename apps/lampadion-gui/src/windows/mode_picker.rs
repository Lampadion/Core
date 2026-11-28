use crate::windows::common::{base_config, frameless};
use lampadion_assets::styles::{RESET, WINDOW_MODE_PICKER};
use lampadion_ui::desktop::shared::controls::WindowControls;
use lampadion_ui::desktop::windows::mode_picker::DesktopModePickerLayout;

use dioxus::desktop::{use_window, Config, LogicalSize};
use dioxus::prelude::*;

pub fn config() -> Config {
	let size = LogicalSize::new(800.0, 900.0);
	base_config("desktop-application__mode-picker-window-root", frameless("Lampadion — Mode selection", size).with_min_inner_size(size))
}

#[component]
pub fn Root() -> Element {
	let window = use_window();

	let drag = use_callback({
		let w = window.clone();
		move |_: ()| w.drag()
	});
	let minimize = use_callback({
		let w = window.clone();
		move |_: ()| w.set_minimized(true)
	});
	let maximize = use_callback({
		let w = window.clone();
		move |_: ()| w.set_maximized(!w.is_maximized())
	});
	let close = use_callback({
		let w = window.clone();
		move |_: ()| w.close()
	});

	use_context_provider(|| WindowControls { drag, minimize, maximize, close });

	rsx! {
		document::Stylesheet { href: RESET }
		document::Stylesheet { href: WINDOW_MODE_PICKER }
		DesktopModePickerLayout {}
	}
}
