use dioxus::prelude::*;
use lampadion_ui::website::App;

fn main() {
	dioxus::logger::init(dioxus::logger::tracing::Level::INFO).expect("logger");

	LaunchBuilder::new()
		.with_cfg(dioxus::web::Config::new().rootname("website-root"))
		.launch(App);
}
