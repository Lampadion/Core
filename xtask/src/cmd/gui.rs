//! `cargo xtask gui-dev`: разработка GUI.

use crate::util::{dx, workspace_root, Res};

pub fn help() -> String {
    "cargo xtask gui-dev [аргументы dx]\n\
     \n\
     Запускает `dx serve --desktop` для apps/lampadion-gui.\n\
     Дополнительные аргументы передаются в dx как есть."
        .into()
}

pub fn run(args: impl Iterator<Item = String>) -> Res<()> {
    let app_dir = workspace_root()?.join("apps/lampadion-gui");
    dx(&app_dir, &["serve", "--desktop", "--package", "lampadion-gui"], args)
}
