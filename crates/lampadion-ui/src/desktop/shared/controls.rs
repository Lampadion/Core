use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct WindowControls {
	pub drag: Callback<()>,
	pub minimize: Callback<()>,
	pub maximize: Callback<()>,
	pub close: Callback<()>,
}

impl WindowControls {
	pub fn noop() -> Self {
		Self {
			drag: Callback::new(|_| {}),
			minimize: Callback::new(|_| {}),
			maximize: Callback::new(|_| {}),
			close: Callback::new(|_| {}),
		}
	}
}

pub fn use_window_controls() -> WindowControls {
	try_use_context::<WindowControls>().unwrap_or_else(WindowControls::noop)
}
