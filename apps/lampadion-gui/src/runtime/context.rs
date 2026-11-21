use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
	inner: Arc<Inner>,
}

struct Inner {
	// позже: клиент IPC к хосту, l10n, настройки
}

impl AppContext {
	pub fn init() -> Self {
		Self { inner: Arc::new(Inner {}) }
	}
}
