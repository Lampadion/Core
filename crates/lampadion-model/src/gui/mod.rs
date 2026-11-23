use std::sync::Arc;

#[derive(Debug)]
pub struct IdError(pub String);

fn validate_id(s: &str) -> Result<Arc<str>, IdError> {
	let ok = s.matches(':').count() == 1
		&& !s.contains('+')
		&& s.split(':').all(|p| !p.is_empty());
	if ok { Ok(s.into()) } else { Err(IdError(format!("bad id: {s}"))) }
}

macro_rules! id_type {
	($name:ident) => {
		#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
		pub struct $name(Arc<str>);

		impl $name {
			pub fn new(s: impl AsRef<str>) -> Result<Self, IdError> {
				validate_id(s.as_ref()).map(Self)
			}
			pub fn as_str(&self) -> &str { &self.0 }
		}
	};
}

id_type!(TabId);
id_type!(TabGroupId);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IconRef {
	Builtin(Arc<str>),
	Extension { ext: Arc<str>, path: Arc<str> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct L10nKey(pub Arc<str>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TabContent {
	Recipes,
	FastKeys,
	AllSymbols,
	Favorites,
	Scripts,
	IME,
	IPA,
	Custom(Arc<str>),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerAfter {
	#[default]
	Auto,
	Never,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabGroupDecl {
	pub id: TabGroupId,
	pub order: i32,
	pub divider_after: DividerAfter,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabDecl {
	pub id: TabId,
	pub group: TabGroupId,
	pub order: i32,
	pub title: L10nKey,
	pub icon: IconRef,
	pub content: TabContent,
}
