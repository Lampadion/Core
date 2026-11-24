use dioxus::prelude::Asset;

use crate::generated_website::ALL;

pub fn asset(rel: &str) -> Option<Asset> {
	ALL.iter().find(|(p, _)| *p == rel).map(|(_, a)| *a)
}

pub fn url(rel: &str) -> Option<String> {
	asset(rel).map(|a| a.to_string())
}
