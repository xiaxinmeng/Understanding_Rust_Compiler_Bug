rust
#![feature(result_map_or)]
pub fn foo() -> bool {
    std::env::var("FOO").map_or(false, |s| s != "BAR")
}
