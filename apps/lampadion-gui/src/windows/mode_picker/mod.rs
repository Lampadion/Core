mod layout;

use self::layout::AppLayout;
use crate::window_scss;
use crate::windows::common::{base_config, frameless};

use dioxus::desktop::{Config, LogicalSize};
use dioxus::prelude::*;

pub fn config() -> Config {
    let size = LogicalSize::new(800.0, 900.0);
    base_config(
        frameless("Lampadion — Mode selection", size).with_min_inner_size(size),
    )
}

#[component]
pub fn Root() -> Element {
	rsx! {
		document::Stylesheet { href: window_scss!("mode_picker") }
		AppLayout {}
	}
}
