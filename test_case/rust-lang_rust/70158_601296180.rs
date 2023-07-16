
> {
  cargo new --lib foo
  cargo new --lib bar
  echo 'baz = { package = "bar", path = "../bar" }' >> foo/Cargo.toml
  echo '[lib]\nproc-macro = true' >> bar/Cargo.toml
  echo '#[proc_macro_attribute] pub fn foo(input: proc_macro::TokenStream, _: proc_macro::TokenStream) -> proc_macro::TokenStream { input }' > bar/src/lib.rs
  echo 'pub use baz::foo;' > foo/src/lib.rs

  cargo build --manifest-path foo/Cargo.toml --target wasm32-unknown-unknown
}

     Created library `foo` package
     Created library `bar` package
   Compiling bar v0.1.0 (/private/var/folders/0p/5yvmrvhj5w3_vy1y8x7dvk7m0000gn/T/tmp.qRqZD3Gc/bar)
   Compiling foo v0.1.0 (/private/var/folders/0p/5yvmrvhj5w3_vy1y8x7dvk7m0000gn/T/tmp.qRqZD3Gc/foo)
thread 'rustc' panicked at 'src/librustc_resolve/imports.rs:896: inconsistent resolution for an import', <::std::macros::panic macros>:2:4
