use dioxus::prelude::*;
use dioxus_html_macro::html;

#[component]
pub fn DekstopInputHintLayout() -> Element {
	html! {
		<div id="app">
			<div class="window-container">
				<main class="main-content">"Подсказка ввода"</main>
			</div>
		</div>
	}
}
