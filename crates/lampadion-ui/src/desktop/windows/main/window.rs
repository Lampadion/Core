use dioxus::prelude::*;
use lampadion_model::gui::TabId;

use super::tabs_builtin::register_builtin_tabs;
use crate::tabs::TabRegistry;

const DEFAULT_TAB: &str = "main:home";

pub fn use_window_context() {
	use_context_provider(|| {
		let mut reg = TabRegistry::new();
		register_builtin_tabs(&mut reg);
		reg
	});

	use_context_provider(|| Signal::new(TabId::new(DEFAULT_TAB).unwrap()));
}
