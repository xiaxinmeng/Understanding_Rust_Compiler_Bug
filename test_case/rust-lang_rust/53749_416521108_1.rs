
macro CSS_PATH() { "C:\\my\\file\\path\\test.css" }

#[cfg(debug_assertions)]
let css = Css::hot_reload(CSS_PATH!).unwrap();

#[cfg(not(debug_assertions))]
let css = Css::new_from_str(include_str!(CSS_PATH!)).unwrap();
