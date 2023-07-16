Rust
#![feature(rustc_attrs)]

fn main() {
    || {
        #[macro_export] #[rustc_macro_transparency = "opaque"] macro_rules! m { () => {} }
    };
}
