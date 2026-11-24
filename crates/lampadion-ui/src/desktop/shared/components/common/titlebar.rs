use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::desktop::shared::components::buttons::WindowControlButtons;
use crate::desktop::shared::controls::use_window_controls;
use lampadion_assets::icons::App;

#[component]
pub fn Titlebar() -> Element {
	let controls = use_window_controls();
	let app_version = env!("CARGO_PKG_VERSION");

	html! {
		<div class="titlebar"
			onmousedown={move |_| controls.drag.call(())}
		>
			<div class="titlebar__left">
				<div class="titlebar__logo">
					{App::Logo.render()}
				</div>
				<div class="titlebar__title">"Lampadion — {app_version}"</div>
			</div>

			<div class="titlebar__right">
				<WindowControlButtons />
			</div>
		</div>
	}
}
