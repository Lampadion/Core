use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::desktop::windows::main::actions::use_main_window_actions;

#[component]
pub fn HomeView() -> Element {
	html! {
		<div class="main-content__lists">
			<OpenModePickerButton />
			<OpenInputHintButton />
		</div>
		<div class="main-content__info-panel"></div>
	}
}

#[component]
fn OpenModePickerButton() -> Element {
	let actions = use_main_window_actions();

	html! {
		<button onclick={move |_| actions.open_mode_picker.call(())}>
			"Выбор режима"
		</button>
	}
}

#[component]
fn OpenInputHintButton() -> Element {
	let actions = use_main_window_actions();

	html! {
		<button onclick={move |_| actions.open_input_hint.call(())}>
			"Подсказки ввода"
		</button>
	}
}
