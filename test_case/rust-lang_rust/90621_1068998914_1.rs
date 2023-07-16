
$ cargo +nightly build
   Compiling xx v0.1.0 (/Users/hans/dev/xx)
error: the target features fp, neon must all be either enabled or disabled together
    --> /Users/hans/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:5191:1
     |
5191 | #[target_feature(enable = "neon")]
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add the missing features in a `target_feature` attribute

error: the target features fp, neon must all be either enabled or disabled together
    --> /Users/hans/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/arm_shared/neon/mod.rs:5721:1
     |
5721 | #[target_feature(enable = "neon")]
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: add the missing features in a `target_feature` attribute

error: could not compile `xx` due to 2 previous errors
