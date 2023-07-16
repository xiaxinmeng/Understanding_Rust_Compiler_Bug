 rust
macro_rules! load_data { ($file:expr) => {
    static DATA: &'static str = include_str!($file);
}}
