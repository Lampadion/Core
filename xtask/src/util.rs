//! Общие помощники: пути, копирование файлов, запуск Dioxus CLI.

use std::{
    env,
    error::Error,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

pub type Res<T> = Result<T, Box<dyn Error>>;

/// ".exe" на Windows, "" на Linux.
pub const EXE: &str = env::consts::EXE_SUFFIX;

/// Корень workspace: на уровень выше `xtask/`.
pub fn workspace_root() -> Res<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "не удалось определить корень workspace".into())
}

/// Копирует `<target_dir>/<name><EXE>` в `dst_dir`.
/// Если `required == false`, отсутствие файла даёт только предупреждение.
pub fn copy_binary(target_dir: &Path, name: &str, dst_dir: &Path, required: bool) -> Res<()> {
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
pub fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
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
pub fn copy_dir_or_warn(src: &Path, dst: &Path) -> Res<()> {
    if src.is_dir() {
        copy_dir(src, dst)?;
    } else {
        eprintln!("предупреждение: {} не найден, создаю пустой каталог", src.display());
        fs::create_dir_all(dst)?;
    }
    Ok(())
}

/// Запускает Dioxus CLI (`dx`) в каталоге приложения.
/// `args` это фиксированные аргументы команды, `extra` это аргументы, переданные пользователем.
pub fn dx(app_dir: &Path, args: &[&str], extra: impl Iterator<Item = String>) -> Res<()> {
    let status = Command::new("dx")
        .current_dir(app_dir)
        .args(args)
        .args(extra)
        .status()
        .map_err(|e| format!("не удалось запустить Dioxus CLI (`dx`): {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`dx {}` завершился с ошибкой", args.join(" ")).into())
    }
}
