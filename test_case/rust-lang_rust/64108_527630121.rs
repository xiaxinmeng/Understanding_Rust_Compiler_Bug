

   Doc-tests‌ alloc‌
error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait.

error: aborting due to previous error


running 459 tests
.................................................................................................... 100/459
.......................................................................................i............ 200/459
..F.FF.............................................................................................. 300/459
.................................................................................................... 400/459
...........................................................
failures:

---- prelude/mod.rs - prelude (line 6) stdout ----
error[E0523]: found two different crates with name `alloc` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
 --> prelude/mod.rs:8:1
  |
5 | extern crate alloc;
  | ^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

Couldn't compile the test.
---- raw_vec.rs - raw_vec::RawVec<T, A>::double (line 261) stdout ----
error[E0523]: found two different crates with name `alloc` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
 --> raw_vec.rs:263:1
  |
4 | extern crate alloc;
  | ^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

Couldn't compile the test.
---- raw_vec.rs - raw_vec::RawVec<T, A>::reserve (line 466) stdout ----
error[E0523]: found two different crates with name `alloc` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
 --> raw_vec.rs:468:1
  |
4 | extern crate alloc;
  | ^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

Couldn't compile the test.

failures:
    prelude/mod.rs - prelude (line 6)
    raw_vec.rs - raw_vec::RawVec<T, A>::double (line 261)
    raw_vec.rs - raw_vec::RawVec<T, A>::reserve (line 466)

test result: FAILED. 455 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out

error:‌ test failed, to rerun pass &#x27;--doc&#x27;‌


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "alloc" "--" "--quiet"
expected success, got: exit code: 101
