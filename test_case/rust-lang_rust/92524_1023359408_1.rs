rust
// build.rs
use std::env::var;

fn main() {
    // 8 MiB stack size
    let stack_size = 8 * 1024 * 1024;
    if var("CARGO_CFG_WINDOWS").is_ok() && var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        // sets the stack size for bins:
        println!("cargo:rustc-link-arg-bins=/stack:{}", stack_size);
    }
}
