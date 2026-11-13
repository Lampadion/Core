use dioxus::prelude::*;

mod app;
mod components;
mod views;

// use dioxus::desktop::tao::window::WindowBuilder;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

#[cfg(target_os = "windows")]
use dioxus::desktop::tao::platform::windows::WindowBuilderExtWindows;


fn main() {
    let mut window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_title("Lampadion")
        .with_inner_size(LogicalSize::new(1280.0, 900.0))
        .with_min_inner_size(LogicalSize::new(1280.0, 900.0));

    #[cfg(target_os = "windows")]
    {
        window = window.with_undecorated_shadow(false);
    }

    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_menu(None)
                .with_background_color((0, 0, 0, 0))
                .with_window(window),
        )
        .launch(app::App);
}
