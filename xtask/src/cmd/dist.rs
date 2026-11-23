//! `cargo xtask dist`: сборка workspace и раскладка релиза.

use std::{env, fs, path::{Path, PathBuf}, process::Command};

use crate::util::{copy_binary, copy_dir, copy_dir_or_warn, workspace_root, Res};

const APP_DIR_NAME: &str = "Lampadion";

// Имена бинарников ([[bin]] name в манифестах приложений).
const LAUNCHER_BIN: &str = "lampadion";
const HOST_BIN: &str = "host";
const CLI_BIN: &str = "cli";
const GUI_BIN: &str = "gui";

pub fn help() -> String {
    format!(
        "cargo xtask dist [--debug] [--no-build] [--out <каталог>]\n\
         \n\
         --debug        профиль debug вместо release\n\
         --no-build     не вызывать cargo build, взять уже собранное\n\
         --out <dir>    каталог результата (по умолчанию dist/{APP_DIR_NAME})\n\
         \n\
         Сборка всегда portable: user/ и маркер `portable` лежат рядом с бинарниками."
    )
}

struct DistOpts {
    release: bool,
    build: bool,
    out: Option<PathBuf>,
}

impl DistOpts {
    fn parse(mut args: impl Iterator<Item = String>) -> Res<Self> {
        let mut o = Self { release: true, build: true, out: None };
        while let Some(a) = args.next() {
            match a.as_str() {
                "--debug" => o.release = false,
                "--no-build" => o.build = false,
                "--out" => {
                    o.out = Some(args.next().ok_or("после --out нужен путь")?.into());
                }
                other => return Err(format!("неизвестный аргумент `{other}`").into()),
            }
        }
        Ok(o)
    }
}

pub fn run(args: impl Iterator<Item = String>) -> Res<()> {
    dist(&DistOpts::parse(args)?)
}

fn dist(opts: &DistOpts) -> Res<()> {
    let root = workspace_root()?;
    let profile = if opts.release { "release" } else { "debug" };
    let target_dir = root.join("target").join(profile);
    let out = opts
        .out
        .clone()
        .unwrap_or_else(|| root.join("dist").join(APP_DIR_NAME));

    if opts.build {
        build_workspace(&root, opts.release)?;
    }

    // Чистый результат: старые файлы не должны переживать пересборку.
    if out.exists() {
        fs::remove_dir_all(&out)?;
    }
    fs::create_dir_all(&out)?;

    // Lampadion/lampadion(.exe): лаунчер в корне
    copy_binary(&target_dir, LAUNCHER_BIN, &out, true)?;

    // Lampadion/bin/{host,cli,gui}
    let bin_dir = out.join("bin");
    fs::create_dir_all(&bin_dir)?;
    copy_binary(&target_dir, HOST_BIN, &bin_dir, true)?;
    copy_binary(&target_dir, CLI_BIN, &bin_dir, true)?;
    // GUI (Dioxus) может ещё не собираться: тогда только предупреждение.
    copy_binary(&target_dir, GUI_BIN, &bin_dir, false)?;

    // Lampadion/data, Lampadion/locale
    copy_dir_or_warn(&root.join("data"), &out.join("data"))?;
    copy_dir_or_warn(&root.join("locale"), &out.join("locale"))?;

    // Lampadion/assets/{images,sounds}: каталоги создаются всегда,
    // содержимое копируется, если в репозитории есть `assets/`.
    let assets_src = root.join("assets");
    for sub in ["images", "sounds"] {
        let dst = out.join("assets").join(sub);
        let src = assets_src.join(sub);
        if src.is_dir() {
            copy_dir(&src, &dst)?;
        } else {
            fs::create_dir_all(&dst)?;
        }
    }

    // Lampadion/extensions: установленные моды
    fs::create_dir_all(out.join("extensions"))?;

    // Пока всегда portable: user/ рядом с бинарниками и маркерный файл.
    // Различие с установленным режимом (каталоги ОС) добавим позже.
    fs::create_dir_all(out.join("user"))?;
    fs::write(out.join("portable"), b"")?;

    println!("Готово: {}", out.display());
    Ok(())
}

fn build_workspace(root: &Path, release: bool) -> Res<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut cmd = Command::new(cargo);
    cmd.current_dir(root).args(["build", "--workspace"]);
    if release {
        cmd.arg("--release");
    }
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err("cargo build завершился с ошибкой".into())
    }
}
