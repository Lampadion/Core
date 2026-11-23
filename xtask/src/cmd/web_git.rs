//! `cargo xtask web-git`: публикация сайта в репозиторий GitHub Pages.

use std::fs;

use crate::{
    git::{git_out, run_git},
    util::{copy_dir, workspace_root, Res},
};

/// Репозиторий GitHub Pages (отдельный от монорепозитория).
const WEB_PAGES_REPO: &str = "https://github.com/Lampadion/Lampadion.github.io.git";
const WEB_PAGES_BRANCH: &str = "main";
/// Результат `dx bundle` относительно корня workspace.
const WEB_PUBLIC_DIR: [&str; 6] = ["target", "dx", "lampadion-website", "release", "web", "public"];

pub fn help() -> String {
    "cargo xtask web-git [--build] [--message <текст>] [--repo <url>]\n\
     \n\
     --build            сначала выполнить web-dist\n\
     --message <текст>  сообщение коммита (по умолчанию «Update website (Core <hash>)»)\n\
     --repo <url>       другой репозиторий вместо Lampadion/Lampadion.github.io\n\
     \n\
     Публикует одним коммитом: история ветки не хранится (force push)."
        .into()
}

struct WebGitOpts {
    build: bool,
    message: Option<String>,
    repo: String,
}

impl WebGitOpts {
    fn parse(mut args: impl Iterator<Item = String>) -> Res<Self> {
        let mut o = Self { build: false, message: None, repo: WEB_PAGES_REPO.into() };
        while let Some(a) = args.next() {
            match a.as_str() {
                "--build" => o.build = true,
                "-m" | "--message" => {
                    o.message = Some(args.next().ok_or("после --message нужно сообщение коммита")?);
                }
                "--repo" => {
                    o.repo = args.next().ok_or("после --repo нужен URL репозитория")?;
                }
                other => return Err(format!("неизвестный аргумент `{other}`").into()),
            }
        }
        Ok(o)
    }
}

pub fn run(args: impl Iterator<Item = String>) -> Res<()> {
    web_git(WebGitOpts::parse(args)?)
}

/// Публикует собранный Website в репозиторий GitHub Pages (ветка `main`).
/// История не сохраняется: в ветке всегда один коммит (force push), как у `gh-pages --no-history`.
fn web_git(opts: WebGitOpts) -> Res<()> {
    let root = workspace_root()?;

    if opts.build {
        super::web::dist(std::iter::empty())?;
    }

    let public = WEB_PUBLIC_DIR.iter().fold(root.clone(), |p, s| p.join(s));
    if !public.is_dir() {
        return Err(format!(
            "не найден {} (сначала `cargo xtask web-dist` или флаг --build)",
            public.display()
        )
        .into());
    }

    // Единственный коммит это и есть вся «история», поэтому в сообщении фиксируем,
    // из какого коммита монорепозитория собран сайт.
    let message = opts.message.unwrap_or_else(|| {
        let rev = git_out(&root, &["rev-parse", "--short", "HEAD"])
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        if rev.is_empty() {
            "Update website".to_string()
        } else {
            format!("Update website (Core {rev})")
        }
    });

    // Пустой репозиторий вместо клона: предыдущие коммиты не нужны.
    let work = root.join("target").join("web-git");
    if work.exists() {
        fs::remove_dir_all(&work)?;
    }
    fs::create_dir_all(&work)?;
    run_git(&work, &["init"])?;
    // Имя ветки задаём напрямую: не зависит от init.defaultBranch и версии git.
    run_git(&work, &["symbolic-ref", "HEAD", &format!("refs/heads/{WEB_PAGES_BRANCH}")])?;
    run_git(&work, &["remote", "add", "origin", &opts.repo])?;

    // CNAME (кастомный домен) берём из текущей публикации, если сборка его не содержит.
    // Для пустого удалённого репозитория fetch просто не удастся, это не ошибка.
    let cname = git_out(&work, &["fetch", "--depth", "1", "origin", WEB_PAGES_BRANCH])
        .ok()
        .and_then(|_| git_out(&work, &["show", "FETCH_HEAD:CNAME"]).ok());

    copy_dir(&public, &work)?;
    if let Some(c) = cname {
        if !work.join("CNAME").exists() {
            fs::write(work.join("CNAME"), c)?;
        }
    }
    // Отключает Jekyll-обработку на GitHub Pages (файлы с `_` в начале имени и т. п.).
    if !work.join(".nojekyll").exists() {
        fs::write(work.join(".nojekyll"), b"")?;
    }

    run_git(&work, &["add", "-A"])?;
    run_git(&work, &["commit", "-m", &message])?;
    run_git(&work, &["push", "--force", "origin", WEB_PAGES_BRANCH])?;

    println!("Опубликовано одним коммитом: {} ({WEB_PAGES_BRANCH})", opts.repo);
    Ok(())
}
