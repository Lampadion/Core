use dioxus::prelude::*;
use dioxus_html_macro::html;

use super::components::navigation::Sidebar;
use super::views::HomeView;
use crate::desktop::shared::components::common::Titlebar;

#[component]
pub fn AppLayout() -> Element {
	html! {
		<div id="app">
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
