mod runtime;
mod shared;
mod windows;

use dioxus::prelude::*;
use runtime::context::AppContext;
use windows::WindowKind;

fn main() {
	let ctx = AppContext::init();
	let first = WindowKind::Main;

	LaunchBuilder::desktop()
		.with_cfg(first.config())
		.with_context(ctx)
		.launch(first.root());
}
