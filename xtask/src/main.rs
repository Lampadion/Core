//! Скрипты проекта Lampadion: `cargo xtask <команда>`.
//!
//! Команды (подробности в `cargo xtask help`):
//!   dist      собрать workspace и разложить релиз в `dist/Lampadion`
//!   icons     сгенерировать иконки всех платформ из lampadion.svg
//!   gui-dev   запустить Dioxus CLI для разработки GUI
//!   web-dev   запустить Dioxus CLI для разработки веб-сайта
//!   web-dist  собрать веб-сайт
//!   web-git   опубликовать веб-сайт в репозиторий GitHub Pages
//!   help      справка

mod cmd;
mod git;
mod help;
mod util;

use std::{env, process::ExitCode};

use util::Res;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("xtask: ошибка: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Res<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("dist") => cmd::dist::run(args),
        Some("icons") => cmd::icons::run(args),
        Some("gui-dev") => cmd::gui::run(args),
        Some("web-dev") => cmd::web::dev(args),
        Some("web-dist") => cmd::web::dist(args),
        Some("web-git") => cmd::web_git::run(args),
        Some("help" | "-h" | "--help") | None => {
            help::print();
            Ok(())
        }
        Some(other) => Err(format!("неизвестная команда `{other}` (см. `cargo xtask help`)").into()),
    }
}
