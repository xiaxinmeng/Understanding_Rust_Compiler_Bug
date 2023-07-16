 rust
#[link(name="iss6919_4", vers="0.1")];

fn no_op() { }
pub static k: extern "Rust" fn() = no_op;
