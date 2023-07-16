rust
> cargo --version
cargo 1.55.0-nightly (495297903 2021-07-01)
> cat src/lib.rs 
pub use reproduction::{ToDiscriminant, TransitiveChild};
use proc_macro::TokenStream;
> cat reproduction/src/lib.rs
#[proc_macro_derive(ToDiscriminant)]
pub fn derive_discriminant(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(TransitiveChild)]
pub fn derive_transitive_child(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
> cargo check --target=wasm32-unknown-unknown
   Compiling reproduction v0.1.0 (/home/joshua/test-rustdoc/creader/reproduction)
    Checking creader v0.1.0 (/home/joshua/test-rustdoc/creader)
    Finished dev [unoptimized + debuginfo] target(s) in 1.20s
