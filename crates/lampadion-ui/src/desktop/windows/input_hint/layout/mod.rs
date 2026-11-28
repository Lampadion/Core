use dioxus::prelude::*;
use dioxus_html_macro::html;

#[component]
pub fn DekstopInputHintLayout() -> Element {
	html! {
		<div id="desktop-application__input-hint-window-root">
			<div class="window-container">
				<main class="main-content">"Подсказка ввода"</main>
			</div>
		</div>
	}
}
