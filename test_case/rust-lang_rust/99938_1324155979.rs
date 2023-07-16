Rust
#![feature(let_chains)]

pub fn foo() {
    if false
        && let Ok(ce) = std::env::current_exe()
        && let Some(ce) = ce.to_str()
    {
        let _ = ce.len();
    }
}
