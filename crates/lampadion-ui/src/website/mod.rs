use dioxus::prelude::*;

use crate::desktop::windows::main::{use_window_context, AppLayout};
use lampadion_assets::icons::FAVICON;
use lampadion_assets::styles::{RESET, WEBSITE_MAIN, WINDOW_MAIN};

#[component]
pub fn App() -> Element {
	rsx! {
		document::Link { rel: "icon", href: FAVICON, r#type: "image/svg+xml" }

		main {
			h1 { "Лампадион" }
			p { "Тестовая сборка веб-сайта на Rust + Dioxus" }

			AppPreview {}
		}
	}
}

#[component]
pub fn AppPreview() -> Element {
	use_window_context();

	rsx! {
		document::Stylesheet { href: RESET }
		document::Stylesheet { href: WEBSITE_MAIN }
		document::Stylesheet { href: WINDOW_MAIN }

		article {
			class: "app-preview",
			"aria-label": "Предпросмотр окна Lampadion",
			AppLayout {}
		}
	}
}
