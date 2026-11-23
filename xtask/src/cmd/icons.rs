//! `cargo xtask icons`: генерация иконок всех платформ из SVG.

use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use svg_to_icons::{
    create_android_icons, create_icns, create_ico, create_png_512, create_pngs,
    create_social_media_png, create_web_targets, svg_to_icon_data,
};

use crate::util::{workspace_root, Res};

pub fn help() -> String {
    "cargo xtask icons [--android] [--android-bg <#RRGGBB>] [--social]\n\
     \n\
     --android              добавить набор иконок Android (bundle/android)\n\
     --android-bg <цвет>    подложка адаптивной иконки Android (по умолчанию белая)\n\
     --social               добавить og-image.png (1200x630)\n\
     \n\
     Источник: crates/lampadion-assets/assets/graphics/icons/app/lampadion.svg\n\
     Результат: crates/lampadion-assets/assets/graphics/icons/bundle"
        .into()
}

struct IconsOpts {
    android: bool,
    android_bg: Option<[u8; 4]>,
    social: bool,
}

impl IconsOpts {
    fn parse(mut args: impl Iterator<Item = String>) -> Res<Self> {
        let mut o = Self { android: false, android_bg: None, social: false };
        while let Some(a) = args.next() {
            match a.as_str() {
                "--android" => o.android = true,
                "--social" => o.social = true,
                "--android-bg" => {
                    let v = args.next().ok_or("после --android-bg нужен цвет (#RRGGBB)")?;
                    o.android_bg = Some(parse_hex_color(&v)?);
                }
                other => return Err(format!("неизвестный аргумент `{other}`").into()),
            }
        }
        Ok(o)
    }
}

/// `#RRGGBB` или `RRGGBB` -> `[r, g, b, 255]`.
fn parse_hex_color(s: &str) -> Res<[u8; 4]> {
    let h = s.trim_start_matches('#');
    if h.len() != 6 || !h.is_ascii() {
        return Err(format!("некорректный цвет `{s}`, ожидается #RRGGBB").into());
    }
    let ch = |i: usize| u8::from_str_radix(&h[i..i + 2], 16);
    Ok([ch(0)?, ch(2)?, ch(4)?, 255])
}

/// Каталог графики: `crates/lampadion-assets/assets/graphics/icons`.
fn icons_dir(root: &Path) -> PathBuf {
    root.join("crates")
        .join("lampadion-assets")
        .join("assets")
        .join("graphics")
        .join("icons")
}

pub fn run(args: impl Iterator<Item = String>) -> Res<()> {
    icons(&IconsOpts::parse(args)?)
}

/// Генерирует ICO, ICNS, PNG-размеры, web-иконки (и по флагам Android и og-image)
/// из `icons/app/lampadion.svg` в `icons/bundle`.
fn icons(opts: &IconsOpts) -> Res<()> {
    let root = workspace_root()?;
    let icons = icons_dir(&root);
    let src = icons.join("app").join("lampadion.svg");
    let out = icons.join("bundle");

    if !src.is_file() {
        return Err(format!("не найден исходный SVG {}", src.display()).into());
    }

    let mut svg = String::new();
    File::open(&src)?.read_to_string(&mut svg)?;

    // Чистый результат: устаревшие размеры не должны переживать регенерацию.
    if out.exists() {
        fs::remove_dir_all(&out)?;
    }
    fs::create_dir_all(&out)?;

    // (размер, тип записи ICNS)
    let sizes = [
        (16, "is32"),
        (32, "il32"),
        (48, "ih32"),
        (64, "ih32"),
        (128, "it32"),
        (256, "ic08"),
        (512, "ic09"),
        (1024, "ic10"),
    ];
    let entries = svg_to_icon_data(&svg, &sizes)?;

    // Desktop: Windows (.ico), macOS (.icns), Linux (PNG разных размеров)
    create_ico(&entries, &out.join("icon.ico"))?;
    create_icns(&entries, &out.join("icon.icns"))?;
    create_pngs(&entries, &sizes, &out)?;
    create_png_512(&svg, &out.join("icon-512.png"))?;

    // Web / PWA: apple-touch-icon, android-chrome-*
    create_web_targets(&svg, &out)?;

    if opts.android {
        // None в масштабе = безопасная зона 66/108.
        create_android_icons(&svg, &out, opts.android_bg, None)?;
    }
    if opts.social {
        create_social_media_png(&svg, &out.join("og-image.png"), 1200, 630, None)?;
    }

    println!("Иконки готовы: {}", out.display());
    Ok(())
}
