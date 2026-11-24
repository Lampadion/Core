use crate::runtime::{context::AppContext, window_manager};
use crate::windows::common::{base_config, frameless};
use crate::windows::WindowKind;
use lampadion_assets::styles::{RESET, WINDOW_MAIN};
use lampadion_ui::desktop::shared::controls::WindowControls;
use lampadion_ui::desktop::windows::main::{use_window_context, DesktopApplicationLayout, MainWindowActions};

use dioxus::desktop::{use_window, Config, LogicalSize};
use dioxus::prelude::*;

pub fn config() -> Config {
	let size = LogicalSize::new(1280.0, 900.0);
	base_config(
		frameless("Lampadion", size).with_min_inner_size(size),
	)
}

#[component]
pub fn Root() -> Element {
	let window = use_window();
	let ctx = use_context::<AppContext>();

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

	let open_mode_picker = use_callback({
		let ctx = ctx.clone();
		move |_: ()| {
			let ctx = ctx.clone();
			spawn(async move {
				window_manager::open(WindowKind::ModePicker, ctx).await;
			});
		}
	});
	let open_input_hint = use_callback({
		let ctx = ctx.clone();
		move |_: ()| {
			let ctx = ctx.clone();
			spawn(async move {
				window_manager::open(WindowKind::InputHint, ctx).await;
			});
		}
	});

	use_context_provider(|| WindowControls { drag, minimize, maximize, close });
	use_context_provider(|| MainWindowActions { open_mode_picker, open_input_hint });
	use_window_context();

	rsx! {
        document::Stylesheet { href: RESET }
        document::Stylesheet { href: WINDOW_MAIN }
        DesktopApplicationLayout {}
    }
}
