rust
fn main() {
  println!("cargo:rustc-cdylib-link-arg=/DEF:./hook/ordinals.def");
}
