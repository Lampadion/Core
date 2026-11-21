use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

#[cfg(target_os = "windows")]
use dioxus::desktop::tao::platform::windows::WindowBuilderExtWindows;

pub fn frameless(title: &str, size: LogicalSize<f64>) -> WindowBuilder {
	let mut window = WindowBuilder::new()
		.with_decorations(false)
		.with_transparent(true)
		.with_title(title)
		.with_inner_size(size);

	#[cfg(target_os = "windows")]
	{
		window = window.with_undecorated_shadow(false);
	}

	window
}

pub fn base_config(window: WindowBuilder) -> Config {
	Config::new()
		.with_menu(None)
		.with_background_color((0, 0, 0, 0))
		.with_window(window)
}
