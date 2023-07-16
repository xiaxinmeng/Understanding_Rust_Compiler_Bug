plain

running 611 tests
.................................................................................................... 100/611
.................................................................................................... 200/611
................................................i............................................F.F.... 300/611
........FF.............F...F........................................................................ 400/611
.................................................................................................... 600/611
...........
failures:


---- src/rc.rs - rc::Rc<T, A>::new_zeroed_in (line 686) stdout ----
error[E0308]: mismatched types
  --> src/rc.rs:693:37
   |
10 | let zero = Rc::<u32>::new_zeroed_in(System);
   |                                     ^^^^^^ expected struct `std::alloc::Global`, found struct `System`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/rc.rs - rc::Rc<T, A>::new_uninit_in (line 644) stdout ----
error[E0308]: mismatched types
  --> src/rc.rs:652:41
   |
11 | let mut five = Rc::<u32>::new_uninit_in(System);
   |                                         ^^^^^^ expected struct `std::alloc::Global`, found struct `System`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/rc.rs - rc::Rc<T, A>::try_new_zeroed_in (line 789) stdout ----
error[E0308]: mismatched types
 --> src/rc.rs:795:41
  |
9 | let zero = Rc::<u32>::try_new_zeroed_in(System)?;
  |                                         ^^^^^^ expected struct `std::alloc::Global`, found struct `System`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/rc.rs - rc::Rc<T, A>::try_new_uninit_in (line 746) stdout ----
error[E0308]: mismatched types
  --> src/rc.rs:753:45
   |
10 | let mut five = Rc::<u32>::try_new_uninit_in(System)?;
   |                                             ^^^^^^ expected struct `std::alloc::Global`, found struct `System`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/rc.rs - rc::Rc<[T], A>::new_uninit_slice_in (line 897) stdout ----
error[E0308]: mismatched types
  --> src/rc.rs:905:54
   |
11 | let mut values = Rc::<[u32]>::new_uninit_slice_in(3, System);
   |                                                      ^^^^^^ expected struct `std::alloc::Global`, found struct `System`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/rc.rs - rc::Rc<[T], A>::new_zeroed_slice_in (line 932) stdout ----
error[E0308]: mismatched types
  --> src/rc.rs:939:50
   |
10 | let values = Rc::<[u32]>::new_zeroed_slice_in(3, System);
   |                                                  ^^^^^^ expected struct `std::alloc::Global`, found struct `System`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:17:17
