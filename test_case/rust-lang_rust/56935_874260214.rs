rust
graphite/editor on  ice-debugging [?] ➜ cat ../proc-macro/src/lib.rs
use proc_macro::TokenStream;

#[proc_macro_derive(ToDiscriminant)]
pub fn derive_discriminant(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(TransitiveChild)]
pub fn derive_transitive_child(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

graphite/editor on  ice-debugging [?] ➜ cat src/lib.rs
pub use proc_macros::{ToDiscriminant, TransitiveChild};

graphite/editor on  ice-debugging [?] ➜ cargo check --target wasm32-unknown-unknown
    Checking graphite-editor-core v0.1.0 (/home/dennis/Projects/rust/Graphite/editor)
thread 'rustc' panicked at 'Failed to get crate data for crate15', compiler/rustc_metadata/src/creader.rs:139:32
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (b3d11f95c 2021-07-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `graphite-editor-core`

To learn more, run the command again with --verbose.
