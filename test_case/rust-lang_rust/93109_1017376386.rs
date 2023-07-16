plain
running 606 tests
.................................................................................................... 100/606
.................................................................................................... 200/606
..........................................................i......................................... 300/606
...F..........F..F.F...F............................................................................ 400/606
..............................................................................F......F.....FF......F 500/606
......
failures:

---- src/rc.rs - rc::Rc<T>::new_uninit (line 446) stdout ----
---- src/rc.rs - rc::Rc<T>::new_uninit (line 446) stdout ----
error[E0599]: no method named `write` found for enum `Option` in the current scope
  --> src/rc.rs:455:24
   |
12 | Rc::get_mut(&mut five).write(5);
   |                        ^^^^^ method not found in `Option<&mut MaybeUninit<u32>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/rc.rs - rc::Rc<T>::try_new_uninit (line 536) stdout ----
error[E0599]: no method named `write` found for enum `Option` in the current scope
  --> src/rc.rs:545:24
   |
12 | Rc::get_mut(&mut five).write(5);
   |                        ^^^^^ method not found in `Option<&mut MaybeUninit<u32>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/rc.rs - rc::Rc<[T]>::new_uninit_slice (line 651) stdout ----
error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
12 | Rc::get_mut(&mut values)[0].write(1);


error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
13 | Rc::get_mut(&mut values)[1].write(2);


error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
14 | Rc::get_mut(&mut values)[2].write(3);

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0608`.
For more information about this error, try `rustc --explain E0608`.
Couldn't compile the test.
---- src/rc.rs - rc::Rc<[mem::MaybeUninit<T>]>::assume_init (line 764) stdout ----
error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
12 | Rc::get_mut(&mut values)[0].write(1);


error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
13 | Rc::get_mut(&mut values)[1].write(2);


error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
14 | Rc::get_mut(&mut values)[2].write(3);

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0608`.
For more information about this error, try `rustc --explain E0608`.
Couldn't compile the test.
---- src/rc.rs - rc::Rc<mem::MaybeUninit<T>>::assume_init (line 727) stdout ----
error[E0599]: no method named `write` found for enum `Option` in the current scope
  --> src/rc.rs:736:24
   |
12 | Rc::get_mut(&mut five).write(5);
   |                        ^^^^^ method not found in `Option<&mut MaybeUninit<u32>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync.rs - sync::Arc<T>::new_uninit (line 432) stdout ----
error[E0599]: no method named `write` found for enum `Option` in the current scope
  --> src/sync.rs:441:25
   |
12 | Arc::get_mut(&mut five).write(5);
   |                         ^^^^^ method not found in `Option<&mut MaybeUninit<u32>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync.rs - sync::Arc<T>::try_new_uninit (line 538) stdout ----
error[E0599]: no method named `write` found for enum `Option` in the current scope
  --> src/sync.rs:547:25
   |
12 | Arc::get_mut(&mut five).write(5);
   |                         ^^^^^ method not found in `Option<&mut MaybeUninit<u32>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync.rs - sync::Arc<[T]>::new_uninit_slice (line 643) stdout ----
error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
12 | Arc::get_mut(&mut values)[0].write(1);


error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
13 | Arc::get_mut(&mut values)[1].write(2);


error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
14 | Arc::get_mut(&mut values)[2].write(3);

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0608`.
For more information about this error, try `rustc --explain E0608`.
Couldn't compile the test.
---- src/sync.rs - sync::Arc<[mem::MaybeUninit<T>]>::assume_init (line 757) stdout ----
error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
12 | Arc::get_mut(&mut values)[0].write(1);


error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
13 | Arc::get_mut(&mut values)[1].write(2);


error[E0608]: cannot index into a value of type `Option<&mut [MaybeUninit<u32>]>`
   |
   |
14 | Arc::get_mut(&mut values)[2].write(3);

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0608`.
For more information about this error, try `rustc --explain E0608`.
Couldn't compile the test.
---- src/sync.rs - sync::Arc<mem::MaybeUninit<T>>::assume_init (line 719) stdout ----
error[E0599]: no method named `write` found for enum `Option` in the current scope
  --> src/sync.rs:728:25
   |
12 | Arc::get_mut(&mut five).write(5);
   |                         ^^^^^ method not found in `Option<&mut MaybeUninit<u32>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:15:56
