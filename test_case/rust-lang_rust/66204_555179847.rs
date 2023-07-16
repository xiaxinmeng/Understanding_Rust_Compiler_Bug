rust
fn main() {
  // Permit undefined symbols to Postgres internal symbols in the `postgres` binary.
  // https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script
  println!("cargo:rustc-cdylib-link-arg=-undefined");
  println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
}
