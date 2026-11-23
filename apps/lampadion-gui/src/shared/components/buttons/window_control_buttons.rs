use dioxus::desktop::use_window;
use dioxus::prelude::*;
use dioxus_html_macro::html;

use lampadion_assets::icons::{
ControlClose, ControlMaximize, ControlMinimize
};

#[component]
pub fn WindowControlButtons() -> Element {
	let window = use_window();

	let w_min = window.clone();
	let w_max = window.clone();
	let w_close = window.clone();

	html! {
		<div class="titlebar__controls">
			<button
				class="titlebar__button titlebar__button--minimize"
				onmousedown={|e| e.stop_propagation()}
				onclick={move |_| {
					w_min.set_minimized(true);
				}}
			>
				<div class="titlebar__button__icon">
					<ControlMinimize />
				</div>
			</button>

			<button
				class="titlebar__button titlebar__button--maximize"
				onmousedown={|e| e.stop_propagation()}
				onclick={move |_| {
					let is_max = w_max.is_maximized();
					w_max.set_maximized(!is_max);
				}}
			>
				<div class="titlebar__button__icon">
					<ControlMaximize />
				</div>
			</button>

			<button
				class="titlebar__button titlebar__button--close"
				onmousedown={|e| e.stop_propagation()}
				onclick={move |_| {
					w_close.close();
				}}
			>
				<div class="titlebar__button__icon">
					<ControlClose />
				</div>
			</button>
		</div>
	}
}
