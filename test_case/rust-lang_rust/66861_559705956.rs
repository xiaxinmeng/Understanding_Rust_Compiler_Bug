rust
fn main() {
    println!("cargo:rustc-link-search=<crypto-path>");
    println!("cargo:rustc-link-lib=static=crypto");
}
