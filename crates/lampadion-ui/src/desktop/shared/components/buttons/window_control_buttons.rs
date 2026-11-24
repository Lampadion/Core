use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::desktop::shared::controls::use_window_controls;
use lampadion_assets::icons::Control;

#[component]
pub fn WindowControlButtons() -> Element {
	let controls = use_window_controls();

	html! {
		<div class="titlebar__controls">
			<button
				class="titlebar__button titlebar__button--minimize"
				onmousedown={|e| e.stop_propagation()}
				onclick={move |_| controls.minimize.call(())}
			>
				<div class="titlebar__button__icon">
					{Control::Minimize.render()}
				</div>
			</button>

			<button
				class="titlebar__button titlebar__button--maximize"
				onmousedown={|e| e.stop_propagation()}
				onclick={move |_| controls.maximize.call(())}
			>
				<div class="titlebar__button__icon">
					{Control::Maximize.render()}
				</div>
			</button>

			<button
				class="titlebar__button titlebar__button--close"
				onmousedown={|e| e.stop_propagation()}
				onclick={move |_| controls.close.call(())}
			>
				<div class="titlebar__button__icon">
					{Control::Close.render()}
				</div>
			</button>
		</div>
	}
}
