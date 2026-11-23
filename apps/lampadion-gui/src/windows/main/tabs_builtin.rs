use lampadion_assets::icons::TabIcon;
use lampadion_model::gui::{
	DividerAfter, L10nKey, TabContent, TabDecl, TabGroupDecl, TabGroupId, TabId,
};
use lampadion_ui::tabs::{TabRegistry, TabSource};

pub fn register_builtin_tabs(reg: &mut TabRegistry) {
	let group = |name: &str| TabGroupId::new(format!("main:{name}")).unwrap();

	for (name, order) in [("home", 0), ("content", 10), ("favorites", 20), ("system", 30)] {
		reg.register_group(
			TabGroupDecl { id: group(name), order, divider_after: DividerAfter::Auto },
			TabSource::Builtin,
		);
	}

	let mut add = |name: &str, grp: &str, order: i32, icon: TabIcon, content: TabContent| {
		reg.register(
			TabDecl {
				id: TabId::new(format!("main:{name}")).unwrap(),
				group: group(grp),
				order,
				title: L10nKey(format!("ui-tab-{name}").into()),
				icon: icon.into(),
				content,
			},
			TabSource::Builtin,
		);
	};

	add("home",      "home",      0,  TabIcon::Hotkey,  TabContent::Custom("main:home".into()));
	add("hotkeys",   "content",   0,  TabIcon::Hotkey,  TabContent::FastKeys);
	add("compose",   "content",   10, TabIcon::Compose, TabContent::Recipes);
	add("scripts",   "content",   20, TabIcon::Script,  TabContent::Scripts);
	add("ime",       "content",   30, TabIcon::Script,  TabContent::IME);
	add("ipa",       "content",   40, TabIcon::Script,  TabContent::IPA);
	add("favorites", "favorites", 0,  TabIcon::Hotkey,  TabContent::Favorites);
	add("settings",  "system",    90, TabIcon::Hotkey,  TabContent::Custom("main:settings".into()));
}
