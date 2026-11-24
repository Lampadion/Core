use dioxus::prelude::*;
use dioxus_html_macro::html;

#[component]
pub fn Header() -> Element {
	html! {
		<header>
			<h1>"Лампадион"</h1>
		</header>
	}
}
