use dioxus::desktop::Config;
use dioxus::prelude::*;

mod common;
pub mod main;
pub mod input_hint;
pub mod mode_picker;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WindowKind {
	Main,
	ModePicker,
	InputHint,
}

impl WindowKind {
	pub fn config(self) -> Config {
		match self {
			Self::Main => main::config(),
			Self::ModePicker => mode_picker::config(),
			Self::InputHint => input_hint::config(),
		}
	}

	pub fn root(self) -> fn() -> Element {
		match self {
			Self::Main => main::Root,
			Self::ModePicker => mode_picker::Root,
			Self::InputHint => input_hint::Root,
		}
	}
}
