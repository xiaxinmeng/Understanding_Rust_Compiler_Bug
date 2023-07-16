plain
   Doc-tests core
error[E0658]: const operands for inline assembly are unstable
  --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/aarch64/armclang.rs:22:20
   |
22 |     asm!("brk {}", const VAL);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error[E0658]: const operands for inline assembly are unstable
  --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/arm/armclang.rs:34:22
   |
34 |     asm!("bkpt #{}", const VAL);
   |
   = note: see issue #72016 <https://github.com/rust-lang/rust/issues/72016> for more information
   = help: add `#![feature(asm_const)]` to the crate attributes to enable


error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:18:20
