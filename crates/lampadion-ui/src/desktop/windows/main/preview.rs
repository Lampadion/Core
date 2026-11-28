use dioxus::prelude::*;

use super::{use_window_context, DesktopApplicationLayout, MainWindowActions};
use crate::desktop::shared::controls::WindowControls;
use lampadion_assets::styles::WINDOW_MAIN;

#[derive(Props, Clone, PartialEq)]
pub struct DesktopApplicationPreviewProps {
	#[props(default = MainWindowActions::noop())]
	pub actions: MainWindowActions,
}

#[component]
pub fn DesktopApplicationPreview(props: DesktopApplicationPreviewProps) -> Element {
	use_context_provider(WindowControls::noop);
	use_context_provider(|| props.actions);
	use_window_context();

	rsx! {
		document::Stylesheet { href: WINDOW_MAIN }

		article {
			class: "desktop-application-preview desktop-application-preview--main-window",
			"aria-label": "Предпросмотр окна Lampadion",
			DesktopApplicationLayout {}
		}
	}
}
