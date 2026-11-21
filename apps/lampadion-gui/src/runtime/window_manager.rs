use std::{cell::RefCell, collections::HashMap};

use dioxus::desktop::{window, DesktopContext};
use dioxus::prelude::*;

use crate::runtime::context::AppContext;
use crate::windows::WindowKind;

thread_local! {
	static OPEN: RefCell<HashMap<WindowKind, DesktopContext>> = RefCell::new(HashMap::new());
}

pub async fn open(kind: WindowKind, ctx: AppContext) {
	let existing = OPEN.with(|m| m.borrow().get(&kind).cloned());
	if let Some(handle) = existing {
			handle.set_visible(true);
			handle.set_focus();
			return;
	}

	let dom = VirtualDom::new(kind.root()).with_root_context(ctx);
	let handle = window().new_window(dom, kind.config()).await;
	OPEN.with(|m| m.borrow_mut().insert(kind, handle));
}

pub fn hide(kind: WindowKind) {
	OPEN.with(|m| {
			if let Some(handle) = m.borrow().get(&kind) {
					handle.set_visible(false);
			}
	});
}
