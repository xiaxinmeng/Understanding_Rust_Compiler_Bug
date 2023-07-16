plain
    Finished release [optimized] target(s) in 12.71s
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 229 tests
..........i.ii....iiF............................................................................... 100/229
...........ii................
failures:
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


---- [run-make] run-make-fulldeps/alloc-no-oom-handling stdout ----

error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling  --edition=2018 --crate-type=rlib ../../../../library/alloc/src/lib.rs --cfg feature=\"external_crate\" --cfg no_global_oom_handling
--- stderr -------------------------------
warning: unused import: `from_fn`
  --> ../../../../library/alloc/src/string.rs:51:18
   |
   |
51 | use core::iter::{from_fn, FusedIterator};
   |
   = note: `#[warn(unused_imports)]` on by default

error[E0277]: the trait bound `str: Clone` is not satisfied
error[E0277]: the trait bound `str: Clone` is not satisfied
    --> ../../../../library/alloc/src/ffi/c_str.rs:1176:38
     |
1176 |     pub fn to_string_lossy(&self) -> Cow<'_, str> {
     |                                      ^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
note: required because of the requirements on the impl of `ToOwned` for `str`
    --> ../../../../library/alloc/src/borrow.rs:84:9
     |
     |
84   | impl<T> ToOwned for T
note: required by a bound in `Cow`
    --> ../../../../library/alloc/src/borrow.rs:183:8
     |
     |
181  | pub enum Cow<'a, B: ?Sized + 'a>
     |          --- required by a bound in this
182  | where
183  |     B: ToOwned,
     |        ^^^^^^^ required by this bound in `Cow`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
make: *** [Makefile:4: all] Error 1



failures:
