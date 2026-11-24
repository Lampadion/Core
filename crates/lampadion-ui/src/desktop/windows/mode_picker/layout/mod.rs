use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::desktop::shared::components::common::Titlebar;

#[component]
pub fn DesktopModePickerLayout() -> Element {
	html! {
		<div id="app">
			<div class="window-container">
				<Titlebar />
				<main class="main-content">"Селектор режима"</main>
			</div>
		</div>
	}
}
