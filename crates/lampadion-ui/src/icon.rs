use std::sync::Arc;

use dioxus::{logger::tracing, prelude::*};
use lampadion_model::gui::IconRef;

#[component]
pub fn Icon(icon: IconRef) -> Element {
	match icon {
		IconRef::Builtin(id) => match lampadion_assets::icons::builtin_svg(&id) {
			Some(svg) => rsx! {
				span { style: "display: contents;", dangerous_inner_html: svg }
			},
			None => {
				tracing::warn!("unknown builtin icon: {id}");
				rsx! {
					span { class: "icon-missing" }
				}
			}
		},
		IconRef::Extension { ext, path } => rsx! {
			ExtensionIcon { ext, path }
		},
	}
}

#[component]
fn ExtensionIcon(ext: Arc<str>, path: Arc<str>) -> Element {
	rsx! {
		img { class: "icon", src: "lampadion-ext://{ext}/{path}" }
	}
}
