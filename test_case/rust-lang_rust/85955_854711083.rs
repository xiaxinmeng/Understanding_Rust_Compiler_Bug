rust
fn main() {
    std::fs::write(
        &std::path::Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("crash.rs"),
        "pub struct A;",
    )
    .unwrap();
}
