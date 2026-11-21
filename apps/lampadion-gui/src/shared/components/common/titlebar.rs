use dioxus::desktop::use_window;
use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::shared::components::buttons::WindowControlButtons;
use crate::shared::icons::AppLogo;

#[component]
pub fn Titlebar() -> Element {
	let window = use_window();
	let app_version = env!("CARGO_PKG_VERSION");

	html! {
		<div class="titlebar"
			onmousedown={move |_| {
				window.drag();
			}}
		>
			<div class="titlebar__left">
				<div class="titlebar__logo">
					<AppLogo />
				</div>
				<div class="titlebar__title">"Lampadion — {app_version}"</div>
			</div>

			<div class="titlebar__right">
				<WindowControlButtons />
			</div>
		</div>
	}
}
