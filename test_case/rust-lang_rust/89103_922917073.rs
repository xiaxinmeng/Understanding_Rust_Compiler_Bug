plain
running 97 tests
iiF.i....i......i.ii.i......i......iii.........................i...i.............i...............
failures:

---- src/builtin.rs - builtin::BARE_TRAIT_OBJECTS (line 1586) stdout ----
error[E0782]: trait objects must include the `dyn` keyword
  |
  |
5 | fn takes_trait_object(_: Box<Trait>) {
  |
  |
help: add `dyn` keyword before this trait
  |
5 | fn takes_trait_object(_: Box<dyn Trait>) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0782`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


Build completed unsuccessfully in 0:22:59
