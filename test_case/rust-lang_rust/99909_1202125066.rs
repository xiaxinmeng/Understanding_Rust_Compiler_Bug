console
$ rustup update nightly
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2022-08-02, rust version 1.64.0-nightly (fe3342816 2022-08-01)
...
  nightly-x86_64-unknown-linux-gnu updated - rustc 1.64.0-nightly (fe3342816 2022-08-01) (from rustc 1.64.0-nightly (9067d5277 2022-07-28))
$ cargo build -p serde_derive
$ nm -D --defined-only -S target/debug/deps/libserde_derive-*.so
0000000000315f10 0000000000000010 D __rustc_proc_macro_decls_284a6d00fbf74d9e__
0000000000000000 0000000000008ca2 N rust_metadata_serde_derive_284a6d00fbf74d9e
