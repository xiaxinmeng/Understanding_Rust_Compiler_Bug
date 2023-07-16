plain
.................................................................................................... 100/598
.................................................................................................... 200/598
................................................i................................................... 300/598
.................................................................................................... 400/598
...............................................................F...........F........................ 500/598
failures:

---- src/sync.rs - sync::Arc<T>::lock_strong_count (line 791) stdout ----
---- src/sync.rs - sync::Arc<T>::lock_strong_count (line 791) stdout ----
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
   |
9  | let x = Arc::new("hello".to_owned());
   |     - help: consider changing this to be mutable: `mut x`
10 | if Arc::lock_strong_count(&mut x) {
   |                           ^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
   |
9  | let x = Arc::new("hello".to_owned());
   |     - help: consider changing this to be mutable: `mut x`
...
12 |     unsafe { *Arc::get_mut_unchecked(&mut x) = "goodbye".to_owned(); }
   |                                      ^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
   |
9  | let x = Arc::new("hello".to_owned());
   |     - help: consider changing this to be mutable: `mut x`
13 |     Arc::unlock_strong_count(&mut x);
   |                              ^^^^^^ cannot borrow as mutable

error: aborting due to 3 previous errors
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0596`.
Couldn't compile the test.
---- src/sync.rs - sync::Arc<T>::unlock_strong_count (line 817) stdout ----
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
   |
9  | let x = Arc::new("hello".to_owned());
   |     - help: consider changing this to be mutable: `mut x`
10 | if Arc::lock_strong_count(&mut x) {
   |                           ^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
   |
9  | let x = Arc::new("hello".to_owned());
   |     - help: consider changing this to be mutable: `mut x`
...
12 |     unsafe { *Arc::get_mut_unchecked(&mut x) = "goodbye".to_owned(); }
   |                                      ^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
   |
9  | let x = Arc::new("hello".to_owned());
   |     - help: consider changing this to be mutable: `mut x`
13 |     Arc::unlock_strong_count(&mut x);
   |                              ^^^^^^ cannot borrow as mutable

error: aborting due to 3 previous errors
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:17:48
