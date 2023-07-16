rust
#![feature(fmt_as_str)]

pub fn a() -> bool {
    format_args!("asdf").as_str().is_some()
}

pub fn b() -> bool {
    format_args!("asdf {}", 1).as_str().is_some()
}
