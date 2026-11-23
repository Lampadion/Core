//! Обёртки над командой `git`.

use std::{path::Path, process::Command};

use crate::util::Res;

/// Запускает `git` с выводом в консоль; ошибка при ненулевом коде.
pub fn run_git(dir: &Path, args: &[&str]) -> Res<()> {
    let status = Command::new("git")
        .current_dir(dir)
        .args(args)
        .status()
        .map_err(|e| format!("не удалось запустить `git` (установлен и есть в PATH?): {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`git {}` завершился с ошибкой", args.join(" ")).into())
    }
}

/// Запускает `git` и возвращает stdout.
pub fn git_out(dir: &Path, args: &[&str]) -> Res<String> {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .map_err(|e| format!("не удалось запустить `git` (установлен и есть в PATH?): {e}"))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(format!("`git {}` завершился с ошибкой", args.join(" ")).into())
    }
}
