use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::desktop::windows::main::DesktopApplicationPreview;

#[component]
pub fn Main() -> Element {
	html! {
		<main>
			<DesktopApplicationPreview />
		</main>
	}
}
