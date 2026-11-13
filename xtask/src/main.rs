//! Скрипты проекта Lampadion: `cargo xtask <команда>`.
//!
//! Команды:
//!   dist   собрать workspace и разложить релиз в `dist/Lampadion`
//!   help   справка

use std::{
    env,
    error::Error,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

type Res<T> = Result<T, Box<dyn Error>>;

const EXE: &str = env::consts::EXE_SUFFIX; // ".exe" на Windows, "" на Linux
const APP_DIR_NAME: &str = "Lampadion";

// Имена бинарников ([[bin]] name в манифестах приложений).
const LAUNCHER_BIN: &str = "lampadion";
const HOST_BIN: &str = "host";
const CLI_BIN: &str = "cli";
const GUI_BIN: &str = "gui";

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
        Some("dist") => dist(&DistOpts::parse(args)?),
        Some("help" | "-h" | "--help") | None => {
            print_help();
            Ok(())
        }
        Some(other) => Err(format!("неизвестная команда `{other}` (см. `cargo xtask help`)").into()),
    }
}

fn print_help() {
    println!(
        "cargo xtask dist [--debug] [--no-build] [--no-frontend] [--out <каталог>]\n\
         \n\
         --debug        профиль debug вместо release\n\
         --no-build     не вызывать cargo build, взять уже собранное\n\
         --no-frontend  не собирать Vue (npm), взять существующий dist/\n\
         --out <dir>   каталог результата (по умолчанию dist/{APP_DIR_NAME})\n\
         \n\
         Сборка всегда portable: user/ и маркер `portable` лежат рядом с бинарниками."
    );
}

// ---------------------------------------------------------------------------
// dist
// ---------------------------------------------------------------------------

struct DistOpts {
    release: bool,
    build: bool,
    frontend: bool,
    out: Option<PathBuf>,
}

impl DistOpts {
    fn parse(mut args: impl Iterator<Item = String>) -> Res<Self> {
        let mut o = Self { release: true, build: true, frontend: true, out: None };
        while let Some(a) = args.next() {
            match a.as_str() {
                "--debug" => o.release = false,
                "--no-build" => o.build = false,
                "--no-frontend" => o.frontend = false,
                "--out" => {
                    o.out = Some(args.next().ok_or("после --out нужен путь")?.into());
                }
                other => return Err(format!("неизвестный аргумент `{other}`").into()),
            }
        }
        Ok(o)
    }
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
    if opts.frontend {
        build_frontend(&frontend_dir(&root))?;
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
    // GUI (Tauri) может ещё не существовать: тогда только предупреждение.
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

    // Lampadion/frontend/gui: собранный Vue-проект (dist/)
    let fe_dist = frontend_dir(&root).join("dist");
    if fe_dist.is_dir() {
        copy_dir(&fe_dist, &out.join("frontend").join("gui"))?;
    } else {
        eprintln!(
            "предупреждение: {} не найден (фронтенд не собран), frontend/gui будет пустым",
            fe_dist.display()
        );
        fs::create_dir_all(out.join("frontend").join("gui"))?;
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

/// Сборка Vue-проекта: установка зависимостей (если нет `node_modules`) и `npm run build`.
/// Если проекта (`package.json`) нет, шаг пропускается с предупреждением.
fn build_frontend(dir: &Path) -> Res<()> {
    if !dir.join("package.json").is_file() {
        eprintln!(
            "предупреждение: {} не содержит package.json, сборка фронтенда пропущена",
            dir.display()
        );
        return Ok(());
    }

    if !dir.join("node_modules").is_dir() {
        // `npm ci` воспроизводимее, но требует package-lock.json.
        let install = if dir.join("package-lock.json").is_file() { "ci" } else { "install" };
        run_npm(dir, &[install])?;
    }
    run_npm(dir, &["run", "build"])
}

fn run_npm(dir: &Path, args: &[&str]) -> Res<()> {
    // На Windows npm это npm.cmd, и Command не находит его под именем `npm`.
    let npm = if cfg!(windows) { "npm.cmd" } else { "npm" };
    let status = Command::new(npm)
        .current_dir(dir)
        .args(args)
        .status()
        .map_err(|e| format!("не удалось запустить `{npm}` (Node.js установлен и есть в PATH?): {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`npm {}` завершился с ошибкой", args.join(" ")).into())
    }
}

// ---------------------------------------------------------------------------
// Вспомогательное
// ---------------------------------------------------------------------------

/// Каталог Vue-проекта в репозитории.
fn frontend_dir(root: &Path) -> PathBuf {
    root.join("frontend").join("lampadion-gui")
}

/// Корень workspace: на уровень выше `xtask/`.
fn workspace_root() -> Res<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "не удалось определить корень workspace".into())
}

/// Копирует `<target_dir>/<name><EXE>` в `dst_dir`.
/// Если `required == false`, отсутствие файла даёт только предупреждение.
fn copy_binary(target_dir: &Path, name: &str, dst_dir: &Path, required: bool) -> Res<()> {
    let file = format!("{name}{EXE}");
    let src = target_dir.join(&file);
    if src.is_file() {
        fs::copy(&src, dst_dir.join(&file))?;
        Ok(())
    } else if required {
        Err(format!("не найден бинарник {}", src.display()).into())
    } else {
        eprintln!("предупреждение: {} не найден, пропускаю", src.display());
        Ok(())
    }
}

/// Рекурсивно копирует каталог (с созданием `dst`).
fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// Копирует каталог, а если источника нет, создаёт пустой и предупреждает.
fn copy_dir_or_warn(src: &Path, dst: &Path) -> Res<()> {
    if src.is_dir() {
        copy_dir(src, dst)?;
    } else {
        eprintln!("предупреждение: {} не найден, создаю пустой каталог", src.display());
        fs::create_dir_all(dst)?;
    }
    Ok(())
}
