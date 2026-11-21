use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::shared::components::common::Titlebar;

#[component]
pub fn AppLayout() -> Element {
	html! {
		<div id="app">
			<div class="window-container">
				<Titlebar />
				<main class="main-content">"Селектор режима"</main>
			</div>
		</div>
	}
}
