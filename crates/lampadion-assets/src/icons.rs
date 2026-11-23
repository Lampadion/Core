use dioxus::prelude::*;

macro_rules! svg {
	($name:ident, $path:literal) => {
		#[component]
		pub fn $name() -> Element {
			rsx! {
				span {
					style: "display: contents;",
					dangerous_inner_html: include_str!(concat!(
						env!("CARGO_MANIFEST_DIR"),
						"/assets/graphics/icons/",
						$path
					))
				}
			}
		}
	};
}

svg!(AppLogo, "app/lampadion.svg");

svg!(TabHotkey, "ui/tab-hotkey.svg");
svg!(TabCompose, "ui/tab-compose.svg");
svg!(TabScript, "ui/tab-script.svg");

svg!(ControlMinimize, "ui/control-minimize.svg");
svg!(ControlMaximize, "ui/control-maximize.svg");
svg!(ControlClose, "ui/control-close.svg");

pub const FAVICON: Asset = asset!("/assets/graphics/icons/app/lampadion.svg");
