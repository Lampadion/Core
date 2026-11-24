use std::collections::BTreeMap;
use std::sync::Arc;

use dioxus::{logger::tracing, prelude::*};
use lampadion_model::gui::{DividerAfter, TabDecl, TabGroupDecl, TabGroupId, TabId};

#[derive(Clone, PartialEq)]
pub enum TabSource {
	Builtin,
	Extension(Arc<str>),
}

#[derive(Clone, PartialEq)]
pub struct TabSpec {
	pub decl: TabDecl,
	pub source: TabSource,
}

#[derive(Clone, PartialEq)]
pub struct GroupSpec {
	pub decl: TabGroupDecl,
	pub source: TabSource,
}

#[derive(Clone, PartialEq)]
pub enum TabEntry {
	Item(TabSpec),
	Divider,
}

#[derive(Clone, Copy)]
pub struct TabRegistry {
	groups: Signal<BTreeMap<TabGroupId, GroupSpec>>,
	tabs: Signal<BTreeMap<TabId, TabSpec>>,
}

impl TabRegistry {
	pub fn new() -> Self {
		Self {
			groups: Signal::new_in_scope(BTreeMap::new(), ScopeId::ROOT),
			tabs: Signal::new_in_scope(BTreeMap::new(), ScopeId::ROOT),
		}
	}

	pub fn register_group(&mut self, decl: TabGroupDecl, source: TabSource) {
		self.groups.write().insert(decl.id.clone(), GroupSpec { decl, source });
	}

	pub fn register(&mut self, decl: TabDecl, source: TabSource) {
		self.tabs.write().insert(decl.id.clone(), TabSpec { decl, source });
	}

	pub fn unregister_source(&mut self, ext: &str) {
		let src = TabSource::Extension(ext.into());
		self.tabs.write().retain(|_, t| t.source != src);
		self.groups.write().retain(|_, g| g.source != src);
	}

	pub fn entries(&self) -> Vec<TabEntry> {
		let groups = self.groups.read();
		let tabs = self.tabs.read();

		for t in tabs.values() {
			if !groups.contains_key(&t.decl.group) {
				tracing::warn!(
					"tab {} refers to unknown group {}",
					t.decl.id.as_str(),
					t.decl.group.as_str()
				);
			}
		}

		let mut ordered: Vec<&GroupSpec> = groups.values().collect();
		ordered.sort_by_key(|g| (g.decl.order, g.decl.id.clone()));

		let buckets: Vec<(&GroupSpec, Vec<&TabSpec>)> = ordered
			.into_iter()
			.map(|g| {
				let mut items: Vec<&TabSpec> =
					tabs.values().filter(|t| t.decl.group == g.decl.id).collect();
				items.sort_by_key(|t| (t.decl.order, t.decl.id.clone()));
				(g, items)
			})
			.filter(|(_, items)| !items.is_empty())
			.collect();

		let mut out = Vec::new();
		let last = buckets.len().saturating_sub(1);
		for (i, (g, items)) in buckets.into_iter().enumerate() {
			out.extend(items.into_iter().map(|t| TabEntry::Item(t.clone())));
			if i != last && g.decl.divider_after != DividerAfter::Never {
				out.push(TabEntry::Divider);
			}
		}
		out
	}
}
