 Rust
#[rustc_mir(graphviz="file.gv")]
pub fn foo<T>(a: &mut T, b: T) {
    *a = b;
}
