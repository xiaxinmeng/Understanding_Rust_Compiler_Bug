
error[E0514]: found crate `unic_langid_impl` compiled by an incompatible version of rustc
 --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/unic-langid-macros-0.9.0/src/lib.rs:2:9
  |
2 | pub use unic_langid_impl::{subtags, LanguageIdentifier};
  |         ^^^^^^^^^^^^^^^^
  |
  = help: please recompile that crate using this compiler (rustc 1.63.0-beta.6 (efd358333 2022-07-16)) (consider running `cargo clean` first)
  = note: the following crate versions were found:
          crate `unic_langid_impl` compiled by rustc 1.63.0-beta.2 (6c1f14289 2022-06-28): /home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/libunic_langid_impl-ca98cc3868906402.rmeta

error[E0514]: found crate `unicode_xid` compiled by an incompatible version of rustc
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.37/src/fallback.rs:16:5
   |
16 | use unicode_xid::UnicodeXID;
   |     ^^^^^^^^^^^
   |
   = help: please recompile that crate using this compiler (rustc 1.63.0-beta.6 (efd358333 2022-07-16)) (consider running `cargo clean` first)
   = note: the following crate versions were found:
           crate `unicode_xid` compiled by rustc 1.63.0-beta.2 (6c1f14289 2022-06-28): /home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/libunicode_xid-9976faca336fa7c2.rmeta
