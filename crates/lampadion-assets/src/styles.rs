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

scss!(WINDOW_MAIN, "windows/main/main.scss");
scss!(WINDOW_INPUT_HINT, "windows/input_hint/input_hint.scss");
scss!(WINDOW_MODE_PICKER, "windows/mode_picker/mode_picker.scss");
