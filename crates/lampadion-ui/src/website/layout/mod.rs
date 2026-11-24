use dioxus::prelude::*;

use crate::website::components::page::{Header, Main, Footer};

#[component]
pub fn WebsiteLayout() -> Element {
	rsx! {
		Header {}
		Main {}
		Footer {}
	}
}
