plain
............................................................................i....i.................. 500/1151
..............................................i...............................ii.................... 600/1151
.................................................................................................... 700/1151
.................................................................................................... 800/1151
...................i...FF.....iii................................................................... 900/1151
...................................................................................iiii............. 1100/1151
...................................................
failures:


---- src/primitive_docs.rs - prim_array (line 520) stdout ----
error[E0277]: `[i32; 3]` is not an iterator
   |
12 | for x in array {
12 | for x in array {
   |          ^^^^^ arrays do not yet implement `IntoIterator`; try using `std::array::IntoIter::new(arr)`
   |
   = help: the trait `Iterator` is not implemented for `[i32; 3]`
   = note: see <https://github.com/rust-lang/rust/pull/65819> for more details
   = note: required because of the requirements on the impl of `IntoIterator` for `[i32; 3]`
   = note: required by `into_iter`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/primitive_docs.rs - prim_array (line 558) stdout ----
error[E0277]: `[i32; 3]` is not an iterator
   |
   |
21 | for item in IntoIterator::into_iter(array).enumerate() {
   |                                     |
   |                                     expected an implementor of trait `IntoIterator`
   |                                     help: consider borrowing here: `&array`
   |
   |
   = note: the trait bound `[i32; 3]: IntoIterator` is not satisfied
   = note: required because of the requirements on the impl of `IntoIterator` for `[i32; 3]`
   = note: required by `into_iter`

error[E0599]: the method `enumerate` exists for array `[i32; 3]`, but its trait bounds were not satisfied
   |
   |
21 | for item in IntoIterator::into_iter(array).enumerate() {
   |                                            ^^^^^^^^^ method cannot be called on `[i32; 3]` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `[i32; 3]: Iterator`
           which is required by `&mut [i32; 3]: Iterator`
           `[i32]: Iterator`
           which is required by `&mut [i32]: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 0 --config /config/nopt-std-config.toml library/std
Build completed unsuccessfully in 0:01:45
