use std::{env, fs, path::{Path, PathBuf}};

fn walk(dir: &Path, root: &Path, out: &mut Vec<PathBuf>) {
	let Ok(rd) = fs::read_dir(dir) else { return };
	let mut entries: Vec<_> = rd.flatten().collect();
	entries.sort_by_key(|e| e.path());
	for e in entries {
		let p = e.path();
		if p.is_dir() {
			walk(&p, root, out);
		} else if matches!(p.extension().and_then(|s| s.to_str()), Some("ron" | "ftl")) {
			out.push(p.strip_prefix(root).unwrap().to_path_buf());
		}
	}
}

fn write_if_changed(path: &Path, code: &str) {
	if fs::read_to_string(path).ok().as_deref() != Some(code) {
		fs::write(path, code).unwrap();
	}
}

fn main() {
	let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let assets = manifest.join("assets");
	println!("cargo:rerun-if-changed=assets");

	for module in ["website", "application"] {
		let mut files = Vec::new();
		walk(&assets.join(module), &assets, &mut files);

		let mut code = String::from(
			"// Generated file by build.rs. Do not edit.\n\
			 use dioxus::prelude::{Asset, manganis};\n\n\
			 pub static ALL: &[(&str, Asset)] = &[\n",
		);
		for f in &files {
			let rel = f.to_string_lossy().replace('\\', "/");
			code += &format!("\t({rel:?}, dioxus::prelude::asset!(\"/assets/{rel}\")),\n");
		}
		code += "];\n";

		write_if_changed(&manifest.join(format!("src/generated_{module}.rs")), &code);
	}
}
