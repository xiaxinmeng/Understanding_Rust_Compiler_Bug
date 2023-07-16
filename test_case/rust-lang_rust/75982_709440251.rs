rust
// src/lib.rs
const _: () = {
    #[path = "foo.rs"]
    mod foo;
};

// src/foo.rs
#[macro_export]
macro_rules! my_macro {
    () => {}
}

// src/main.rs
fn main() {
    weird_path::my_macro!();
}
