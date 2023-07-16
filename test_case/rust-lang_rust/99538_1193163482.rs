
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.04s
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.26s
Checking stage0 std test/bench/example targets (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.16s
Checking stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Checking proc-macro2 v1.0.37
    Checking unic-langid-macros v0.9.0
    Checking rustc_ast v0.0.0 (/home/r/src/rust/rustc/compiler/rustc_ast)
    Checking rustc_error_messages v0.0.0 (/home/r/src/rust/rustc/compiler/rustc_error_messages)
    Checking rustc_target v0.0.0 (/home/r/src/rust/rustc/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/home/r/src/rust/rustc/compiler/rustc_feature)
    Checking rustc_log v0.0.0 (/home/r/src/rust/rustc/compiler/rustc_log)
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

error: could not compile `unic-langid-macros` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `proc-macro2` due to previous error
Build completed unsuccessfully in 0:00:04
