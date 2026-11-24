
// Sample

// pub fn use_text_resource(url: impl Fn() -> Option<String> + 'static) -> Resource<Result<String, String>> {
// 	use_resource(move || {
// 		let url = url();
// 		async move {
// 			let url = url.ok_or_else(|| "файл не найден".to_string())?;
// 			load_text(&url).await
// 		}
// 	})
// }
