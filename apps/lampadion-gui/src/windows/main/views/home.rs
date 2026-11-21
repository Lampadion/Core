use dioxus::prelude::*;

#[component]
pub fn HomeView() -> Element {
    rsx! {
			div { class: "main-content__lists",
				OpenModePickerButton {}
				OpenInputHintButton {}
			}
			div { class: "main-content__info-panel" }
		}
}

// Testing

use crate::runtime::{context::AppContext, window_manager};
use crate::windows::WindowKind;

#[component]
fn OpenModePickerButton() -> Element {
    let ctx = use_context::<AppContext>();

    rsx! {
			button {
				onclick: move |_| {
				    let ctx = ctx.clone();
				    spawn(async move {
				        window_manager::open(WindowKind::ModePicker, ctx).await;
				    });
				},
				"Выбор режима"
			}
		}
}

#[component]
fn OpenInputHintButton() -> Element {
    let ctx = use_context::<AppContext>();

    rsx! {
			button {
				onclick: move |_| {
				    let ctx = ctx.clone();
				    spawn(async move {
				        window_manager::open(WindowKind::InputHint, ctx).await;
				    });
				},
				"Подсказки ввода"
			}
		}
}
