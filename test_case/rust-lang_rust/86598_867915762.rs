plain
.................................................................................................... 2600/2994
............................i...............................................i................i...... 2700/2994
...............i.....................i.....................i.....................i.................. 2800/2994
..............i.....................i.....................i.....................i................... 2900/2994
..i..................................F.F.F..FF................................................

---- src/task/poll.rs - task::poll::Poll<T>::from (line 222) stdout ----
---- src/task/poll.rs - task::poll::Poll<T>::from (line 222) stdout ----
error[E0252]: the name `Poll` is defined multiple times
  |
4 | use core::task::Poll;
4 | use core::task::Poll;
  |     ---------------- previous import of the type `Poll` here
6 | use core::task::Poll;
6 | use core::task::Poll;
  |     ^^^^^^^^^^^^^^^^ `Poll` reimported here
  |
  = note: `Poll` must be defined only once in the type namespace of this block

error: the item `Poll` is imported redundantly
  |
4 | use core::task::Poll;
4 | use core::task::Poll;
  |     ---------------- the item `Poll` is already imported here
6 | use core::task::Poll;
  |     ^^^^^^^^^^^^^^^^
  |
note: the lint level is defined here
---
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0252`.
Couldn't compile the test.
---- src/task/poll.rs - task::poll::Poll<T>::is_pending (line 82) stdout ----
error[E0573]: expected type, found variant `Poll::Ready`
  |
  |
6 | let x: Poll::Ready<u32> = Some(2);
  |
help: consider importing one of these items instead
  |
3 | use core::future::Ready;
3 | use core::future::Ready;
  |
3 | use std::future::Ready;
  |

error[E0573]: expected type, found variant `Poll::Pending`
  |
  |
9 | let x: Poll::Pending<u32> = Poll::Pending;
  |
help: consider importing one of these items instead
  |
3 | use core::future::Pending;
---
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0573`.
Couldn't compile the test.
---- src/task/poll.rs - task::poll::Poll<T>::is_ready (line 60) stdout ----
error[E0573]: expected type, found variant `Poll::Ready`
  |
  |
6 | let x: Poll::Ready<u32> = Some(2);
  |
help: consider importing one of these items instead
  |
3 | use core::future::Ready;
3 | use core::future::Ready;
  |
3 | use std::future::Ready;
  |

error[E0573]: expected type, found variant `Poll::Pending`
  |
  |
9 | let x: Poll::Pending<u32> = Poll::Pending;
  |
help: consider importing one of these items instead
  |
3 | use core::future::Pending;
---
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0573`.
Couldn't compile the test.
---- src/task/poll.rs - task::poll::Poll<Option<Result<T, E>>>::map_err (line 195) stdout ----
 --> src/task/poll.rs:198:44
  |
  |
6 | let res: Poll<Result<u8, _>> = Poll::Ready(Some("oops".parse()));
  |                                            |
  |                                            expected enum `Result`, found enum `Option`
  |                                            expected enum `Result`, found enum `Option`
  |                                            help: try using a variant of the expected enum: `Err(Some("oops".parse()))`
  |
  = note: expected enum `Result<u8, _>`
             found enum `Option<Result<_, _>>`
error[E0308]: mismatched types
 --> src/task/poll.rs:200:1
  |
  |
8 | assert_eq!(res, Poll::Ready(Some(Err(0))));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `Option`
  |
  = note: expected enum `Poll<Result<u8, u8>>`
             found enum `Poll<Option<Result<_, {integer}>>>`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/task/poll.rs - task::poll::Poll<Option<Result<T, E>>>::map_ok (line 165) stdout ----
 --> src/task/poll.rs:168:44
  |
  |
6 | let res: Poll<Result<u8, _>> = Poll::Ready(Some("12".parse()));
  |                                            |
  |                                            expected enum `Result`, found enum `Option`
  |                                            expected enum `Result`, found enum `Option`
  |                                            help: try using a variant of the expected enum: `Err(Some("12".parse()))`
  |
  = note: expected enum `Result<u8, _>`
             found enum `Option<Result<_, _>>`
error[E0308]: mismatched types
 --> src/task/poll.rs:170:1
  |
  |
8 | assert_eq!(squared, Poll::Ready(Some(Ok(144))));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found enum `Option`
  |
  = note: expected enum `Poll<Result<u8, _>>`
             found enum `Poll<Option<Result<{integer}, _>>>`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/task/poll.rs - task::poll::Poll<Option<Result<T, E>>>::map_err (line 195)
    src/task/poll.rs - task::poll::Poll<Option<Result<T, E>>>::map_ok (line 165)
    src/task/poll.rs - task::poll::Poll<T>::is_pending (line 82)
    src/task/poll.rs - task::poll::Poll<T>::is_ready (line 60)

test result: FAILED. 2962 passed; 5 failed; 27 ignored; 0 measured; 0 filtered out; finished in 50.08s
test result: FAILED. 2962 passed; 5 failed; 27 ignored; 0 measured; 0 filtered out; finished in 50.08s

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:24
