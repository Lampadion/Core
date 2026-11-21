macro_rules! svg {
    ($path:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/graphics/icons/", $path))
    };
}

pub const APP_LOGO: &str = svg!("app/lampadion.svg");

pub const TAB_HOTKEY: &str = svg!("ui/tab-hotkey.svg");
pub const TAB_COMPOSE: &str = svg!("ui/tab-compose.svg");
pub const TAB_SCRIPT: &str = svg!("ui/tab-script.svg");

pub const CONTROL_MINIMIZE: &str = svg!("ui/control-minimize.svg");
pub const CONTROL_MAXIMIZE: &str = svg!("ui/control-maximize.svg");
pub const CONTROL_CLOSE: &str = svg!("ui/control-close.svg");
