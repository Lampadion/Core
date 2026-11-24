use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct MainWindowActions {
	pub open_mode_picker: Callback<()>,
	pub open_input_hint: Callback<()>,
}

impl MainWindowActions {
	pub fn noop() -> Self {
		Self {
			open_mode_picker: Callback::new(|_| {}),
			open_input_hint: Callback::new(|_| {}),
		}
	}
}

pub fn use_main_window_actions() -> MainWindowActions {
	try_use_context::<MainWindowActions>().unwrap_or_else(MainWindowActions::noop)
}
