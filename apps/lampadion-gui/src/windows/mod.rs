use dioxus::desktop::Config;
use dioxus::prelude::*;

mod common;
pub mod main;
// pub mod compose_hint;
pub mod mode_picker;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WindowKind {
	Main,
	// ComposeHint,
	ModePicker,
}

impl WindowKind {
	pub fn config(self) -> Config {
		match self {
			Self::Main => main::config(),
			Self::ModePicker => mode_picker::config()
		}
	}

	pub fn root(self) -> fn() -> Element {
		match self {
			Self::Main => main::Root,
			Self::ModePicker => mode_picker::Root,
		}
	}
}
