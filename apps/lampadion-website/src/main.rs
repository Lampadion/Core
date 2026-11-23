use dioxus::prelude::*;
use lampadion_assets::icons::FAVICON;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "icon", href: FAVICON, r#type: "image/svg+xml" }

		main {
			h1 { "Лампадион" }
			p { "Тестовая сборка веб-сайта на Rust + Dioxus" }
		}
	}
}
