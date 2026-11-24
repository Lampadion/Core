use dioxus::prelude::*;

#[macro_export]
macro_rules! scss {
	($name:ident, $path:literal) => {
		pub const $name: Asset = asset!(concat!(
			"/assets/styles/",
			$path
		));
	};
}

scss!(RESET, "kit/utils/reset.scss");

scss!(WINDOW_MAIN, "desktop/windows/main/main.scss");
scss!(WINDOW_INPUT_HINT, "desktop/windows/input_hint/input_hint.scss");
scss!(WINDOW_MODE_PICKER, "desktop/windows/mode_picker/mode_picker.scss");

scss!(WEBSITE_MAIN, "website/main.scss");
