//! `cargo xtask web-dev` и `web-dist`: разработка и сборка веб-сайта.
use std::fs;

use crate::util::{dx, workspace_root, Res};

const WEB_APP_DIR: &str = "apps/lampadion-website";
const WEB_PACKAGE: &str = "lampadion-website";
const WEB_PROFILE: &str = "web-release";
const WEB_PUBLIC_DIR: &str = "target/dx/lampadion-website/release/web/public";

pub fn help() -> String {
    "cargo xtask web-dev [аргументы dx]\n\
     \n\
     Запускает `dx serve --web` для apps/lampadion-website.\n\
     \n\
     cargo xtask web-dist [аргументы dx]\n\
     \n\
     Собирает статическую версию сайта (`dx bundle --web`, профиль web-release).\n\
     Дополнительные аргументы передаются в dx как есть."
        .into()
}

/// Запускает Dioxus CLI для разработки Website.
pub fn dev(args: impl Iterator<Item = String>) -> Res<()> {
    let app_dir = workspace_root()?.join(WEB_APP_DIR);
    dx(&app_dir, &["serve", "--web", "--package", WEB_PACKAGE], args)
}

/// Собирает статическую версию Website.
pub fn dist(args: impl Iterator<Item = String>) -> Res<()> {
    let root = workspace_root()?;

    // Имена wasm и js содержат хэш содержимого, поэтому без очистки
    // старые версии остаются рядом с новыми.
    let public = root.join(WEB_PUBLIC_DIR);
    if public.exists() {
        fs::remove_dir_all(&public)?;
    }

    let app_dir = root.join(WEB_APP_DIR);
    dx(
        &app_dir,
        &["bundle", "--web", "--package", WEB_PACKAGE, "--profile", WEB_PROFILE],
        args,
    )
}
