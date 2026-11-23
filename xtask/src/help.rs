//! Справка: собирается из описаний, которые лежат рядом с каждой командой.

use crate::cmd::{dist, gui, icons, web, web_git};

pub fn print() {
    let parts = [
        dist::help(),
        icons::help(),
        gui::help(),
        web::help(),
        web_git::help(),
    ];
    println!("{}", parts.join("\n\n"));
}
