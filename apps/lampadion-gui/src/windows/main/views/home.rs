use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::runtime::{context::AppContext, window_manager};
use crate::windows::WindowKind;

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
	let ctx = use_context::<AppContext>();

	html! {
		<button
			onclick={move |_| {
				let ctx = ctx.clone();
				spawn(async move {
						window_manager::open(WindowKind::ModePicker, ctx).await;
				});
			}}
		>
			"Выбор режима"
		</button>
	}
}

#[component]
fn OpenInputHintButton() -> Element {
	let ctx = use_context::<AppContext>();

	html! {
		<button
			onclick={move |_| {
				let ctx = ctx.clone();
				spawn(async move {
					window_manager::open(WindowKind::InputHint, ctx).await;
				});
			}}
		>
			"Подсказки ввода"
		</button>
	}
}
