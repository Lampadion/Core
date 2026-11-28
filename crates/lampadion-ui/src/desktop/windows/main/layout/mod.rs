use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::desktop::windows::main::components::navigation::Sidebar;
use crate::desktop::windows::main::views::HomeView;
use crate::desktop::shared::components::common::Titlebar;

#[component]
pub fn DesktopApplicationLayout() -> Element {
	html! {
		<div id="desktop-application__main-window-root">
			<div class="window-container">
				<Titlebar />
				<main class="main-content">
					<div class="main-content__left-island">
						<Sidebar />
					</div>
					<div class="main-content__right-island">
						<HomeView />
					</div>
				</main>
			</div>
		</div>
	}
}
