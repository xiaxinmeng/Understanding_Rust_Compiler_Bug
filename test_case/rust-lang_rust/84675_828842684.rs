plain
....................................................................................iiii............ 1100/1152
....................................................
failures:

---- src/primitive_docs.rs - prim_array (line 615) stdout ----
error[E0277]: `[i32; 3]` is not an iterator
   |
21 | for item in array {
21 | for item in array {
   |             ^^^^^ arrays do not yet implement `IntoIterator`; try using `std::array::IntoIter::new(arr)`
   |
   = help: the trait `Iterator` is not implemented for `[i32; 3]`
   = note: see <https://github.com/rust-lang/rust/pull/65819> for more details
   = note: required because of the requirements on the impl of `IntoIterator` for `[i32; 3]`
   = note: required by `into_iter`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 0 --config /config/nopt-std-config.toml library/std
Build completed unsuccessfully in 0:01:43
