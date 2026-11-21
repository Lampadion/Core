use dioxus::prelude::*;
use dioxus_html_macro::html;

#[component]
pub fn AppLayout() -> Element {
	html! {
		<div id="app">
			<div class="window-container">
				<main class="main-content">"Подсказка ввода"</main>
			</div>
		</div>
	}
}
