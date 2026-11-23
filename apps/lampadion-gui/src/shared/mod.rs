// pub mod icons;
pub mod components;

#[macro_export]
macro_rules! window_scss {
    ($name:literal) => {
        asset!(concat!(
            "/assets/styles/windows/",
            $name,
            "/",
            $name,
            ".scss"
        ))
    };
}
