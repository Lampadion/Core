use dioxus::prelude::*;

#[component]
pub fn AppLayout() -> Element {
	rsx! {
		div { id: "app",
			div { class: "window-container",
				main { class: "main-content", "Подсказка ввода" }
			}
		}
	}
}
