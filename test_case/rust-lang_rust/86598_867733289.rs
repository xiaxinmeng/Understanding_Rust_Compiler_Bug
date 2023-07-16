plain
.................................................................................................... 2600/2994
............................i...............................................i................i...... 2700/2994
...............i.....................i.....................i.....................i.................. 2800/2994
..............i.....................i.....................i.....................i................... 2900/2994
..i............................F..F..FFF..FF..................................................


---- src/task/poll.rs - task::poll::Poll<Option<Result<T, E>>>::map_err (line 183) stdout ----
error[E0412]: cannot find type `Poll` in this scope
  |
  |
4 | let res: Poll<Result<u8, _>> = Poll::Ready(Some("oops".parse()));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
3 | use core::task::Poll;
  |
3 | use std::task::Poll;
  |

error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:184:32
  |
4 | let res: Poll<Result<u8, _>> = Poll::Ready(Some("oops".parse()));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
3 | use core::task::Poll;
  |
3 | use std::task::Poll;
  |

error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:186:17
  |
6 | assert_eq!(res, Poll::Ready(Some(Err(0))));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
---

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
Couldn't compile the test.
---- src/task/poll.rs - task::poll::Poll<Result<T, E>>::map_err (line 128) stdout ----
error[E0412]: cannot find type `Poll` in this scope
  |
  |
4 | let res: Poll<Result<u8, _>> = Poll::Ready("oops".parse());
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
3 | use core::task::Poll;
  |
3 | use std::task::Poll;
  |

error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:129:32
  |
4 | let res: Poll<Result<u8, _>> = Poll::Ready("oops".parse());
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
3 | use core::task::Poll;
  |
3 | use std::task::Poll;
  |

error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:131:17
  |
6 | assert_eq!(res, Poll::Ready(Err(0)));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
---

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
Couldn't compile the test.
---- src/task/poll.rs - task::poll::Poll<T>::is_pending (line 78) stdout ----
error[E0433]: failed to resolve: use of undeclared type `Poll`
  |
  |
4 | let x: Poll::Ready<u32> = Some(2);
  |        ^^^^ use of undeclared type `Poll`
error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:82:8
  |
  |
7 | let x: Poll::Pending<u32> = Poll::Pending;
  |        ^^^^ use of undeclared type `Poll`
error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:82:29
  |
  |
7 | let x: Poll::Pending<u32> = Poll::Pending;
  |                             ^^^^ use of undeclared type `Poll`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.
Couldn't compile the test.
---- src/task/poll.rs - task::poll::Poll<Option<Result<T, E>>>::map_ok (line 155) stdout ----
error[E0412]: cannot find type `Poll` in this scope
  |
  |
4 | let res: Poll<Result<u8, _>> = Poll::Ready(Some("12".parse()));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
3 | use core::task::Poll;
  |
3 | use std::task::Poll;
  |

error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:156:32
  |
4 | let res: Poll<Result<u8, _>> = Poll::Ready(Some("12".parse()));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
3 | use core::task::Poll;
  |
3 | use std::task::Poll;
  |

error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:158:21
  |
6 | assert_eq!(squared, Poll::Ready(Some(Ok(144))));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
---
---- src/task/poll.rs - task::poll::Poll<T>::is_ready (line 58) stdout ----
error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:59:8
  |
4 | let x: Poll::Ready<u32> = Some(2);
  |        ^^^^ use of undeclared type `Poll`
error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:62:8
  |
  |
7 | let x: Poll::Pending<u32> = Poll::Pending;
  |        ^^^^ use of undeclared type `Poll`
error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:62:29
  |
  |
7 | let x: Poll::Pending<u32> = Poll::Pending;
  |                             ^^^^ use of undeclared type `Poll`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.
Couldn't compile the test.
---- src/task/poll.rs - task::poll::Poll<Result<T, E>>::map_ok (line 102) stdout ----
error[E0412]: cannot find type `Poll` in this scope
  |
  |
4 | let res: Poll<Result<u8, _>> = Poll::Ready("12".parse());
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
3 | use core::task::Poll;
  |
3 | use std::task::Poll;
  |

error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:103:32
  |
4 | let res: Poll<Result<u8, _>> = Poll::Ready("12".parse());
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
3 | use core::task::Poll;
  |
3 | use std::task::Poll;
  |

error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:105:21
  |
6 | assert_eq!(squared, Poll::Ready(Ok(144)));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
---
---- src/task/poll.rs - task::poll::Poll<T>::map (line 36) stdout ----
error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:37:24
  |
4 | let poll_some_string = Poll::Ready(String::from("Hello, World!"));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
3 | use core::task::Poll;
  |
3 | use std::task::Poll;
  |

error[E0433]: failed to resolve: use of undeclared type `Poll`
 --> src/task/poll.rs:41:27
  |
8 | assert_eq!(poll_some_len, Poll::Ready(13));
  |
help: consider importing one of these items
  |
3 | use core::task::Poll;
---
For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.

failures:
    src/task/poll.rs - task::poll::Poll<Option<Result<T, E>>>::map_err (line 183)
    src/task/poll.rs - task::poll::Poll<Option<Result<T, E>>>::map_ok (line 155)
    src/task/poll.rs - task::poll::Poll<Result<T, E>>::map_err (line 128)
    src/task/poll.rs - task::poll::Poll<Result<T, E>>::map_ok (line 102)
    src/task/poll.rs - task::poll::Poll<T>::is_pending (line 78)
    src/task/poll.rs - task::poll::Poll<T>::map (line 36)

test result: FAILED. 2960 passed; 7 failed; 27 ignored; 0 measured; 0 filtered out; finished in 48.14s


error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:10
