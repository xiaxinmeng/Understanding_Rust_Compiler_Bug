plain
test src/builtin.rs - builtin::WARNINGS (line 709) ... ok

failures:

---- src/builtin.rs - builtin::UNSUPPORTED_NAKED_FUNCTIONS (line 2729) stdout ----
warning: naked functions must contain a single asm block
  |
  |
6 | / pub extern "sysv64" fn f() -> u32 {
7 | |     42
  | |     -- non-asm is unsupported in naked functions
  | |_^
  |
  = note: `#[warn(unsupported_naked_functions)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

error[E0570]: `"sysv64"` is not a supported ABI for the current target
  |
  |
6 | pub extern "sysv64" fn f() -> u32 {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0570`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "14" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--"


Build completed unsuccessfully in 0:28:51
