use crate::windows::common::{base_config, frameless};
use lampadion_assets::styles::{RESET, WINDOW_INPUT_HINT};
use lampadion_ui::desktop::windows::input_hint::DekstopInputHintLayout;

use dioxus::desktop::{use_window, Config, LogicalSize};
use dioxus::prelude::*;

pub fn config() -> Config {
	let size = LogicalSize::new(400.0, 200.0);
	base_config(
		frameless("Lampadion — Input hint", size).with_min_inner_size(size),
	)
}

#[component]
pub fn Root() -> Element {
	rsx! {
        document::Stylesheet { href: RESET }
        document::Stylesheet { href: WINDOW_INPUT_HINT }
        DekstopInputHintLayout {}
    }
}
