
use dioxus::prelude::*;

use crate::desktop::windows::main::{use_window_context, DesktopApplicationLayout};
use lampadion_assets::styles::{WINDOW_MAIN};

#[component]
pub fn DesktopApplicationPreview() -> Element {
	use_window_context();

	rsx! {
		document::Stylesheet { href: WINDOW_MAIN }

		article {
			class: "app-preview",
			"aria-label": "Предпросмотр окна Lampadion",
			DesktopApplicationLayout {}
				// LatinData {}
		}
	}
}

// async fn load_text(url: &str) -> Result<String, String> {
// 	gloo_net::http::Request::get(url)
// 		.send().await.map_err(|e| e.to_string())?
// 		.text().await.map_err(|e| e.to_string())
// }

// #[component]
// fn LatinData() -> Element {
// 	let ron = use_resource(|| async {
// 		let url = data::web::url("website/characters/latin.ron")
// 			.ok_or_else(|| "файл не найден".to_string())?;
// 		load_text(&url).await
// 	});

// 	match &*ron.read() {
// 		None => rsx! {
// 			p { "Загрузка…" }
// 		},
// 		Some(Err(e)) => rsx! {
// 			p { "Ошибка: {e}" }
// 		},
// 		Some(Ok(text)) => rsx! {
// 			p { "Загружено байт: {text.len()}" }
// 			p { {text.as_str()} }
// 		},
// 	}
// }
