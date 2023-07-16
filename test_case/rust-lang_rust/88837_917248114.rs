plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: an `#[unstable]` annotation here has no effect
     |
     |
1164 | #[unstable(feature = "maybe_uninit_array_index", issue = "none")]
     |
     |
     = note: `#[deny(ineffective_unstable_trait_impl)]` on by default


error: an `#[unstable]` annotation here has no effect
     |
     |
1174 | #[unstable(feature = "maybe_uninit_array_index", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1192 | #[unstable(feature = "maybe_uninit_array_index", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information


error: an `#[unstable]` annotation here has no effect
     |
     |
1203 | #[unstable(feature = "maybe_uninit_array_index", issue = "none")]
     |
     = note: see issue #55436 <https://github.com/rust-lang/rust/issues/55436> for more information

error: could not compile `core` due to 4 previous errors
