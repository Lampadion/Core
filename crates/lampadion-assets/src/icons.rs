use dioxus::prelude::*;
use lampadion_model::gui::IconRef;

pub const FAVICON: Asset = asset!("/assets/graphics/icons/app/lampadion.svg");

macro_rules! icons {
	(
		$(
			$name:ident {
				$($variant:ident => ($id:literal, $path:literal)),* $(,)?
			}
		)*
	) => {
		$(
			#[derive(Clone, Copy, Debug, PartialEq, Eq)]
			pub enum $name {
				$($variant),*
			}

			impl $name {
				pub const ALL: &'static [$name] = &[$(Self::$variant),*];

				pub fn id(&self) -> &'static str {
					match self { $(Self::$variant => $id),* }
				}

				pub fn from_id(id: &str) -> Option<Self> {
					match id { $($id => Some(Self::$variant),)* _ => None }
				}

				pub fn svg(&self) -> &'static str {
					match self {
						$(Self::$variant => include_str!(concat!(
							env!("CARGO_MANIFEST_DIR"),
							"/assets/graphics/icons/",
							$path
						))),*
					}
				}

				pub fn render(&self) -> Element {
					rsx! {
						span { style: "display: contents;", dangerous_inner_html: self.svg() }
					}
				}
			}

			impl From<$name> for IconRef {
				fn from(i: $name) -> Self { IconRef::Builtin(i.id().into()) }
			}
		)*
	};
}

icons! {
	App {
		Logo => ("app.logo", "app/lampadion.svg"),
	}
	Control {
		Minimize => ("control.minimize", "ui/control-minimize.svg"),
		Maximize => ("control.maximize", "ui/control-maximize.svg"),
		Close    => ("control.close",    "ui/control-close.svg"),
	}
	TabIcon {
		Hotkey  => ("tab.hotkey",  "ui/tab-hotkey.svg"),
		Compose => ("tab.compose", "ui/tab-compose.svg"),
		Script  => ("tab.script",  "ui/tab-script.svg"),
	}
}

pub fn builtin_svg(id: &str) -> Option<&'static str> {
	App::from_id(id).map(|i| i.svg())
		.or_else(|| Control::from_id(id).map(|i| i.svg()))
		.or_else(|| TabIcon::from_id(id).map(|i| i.svg()))
}
