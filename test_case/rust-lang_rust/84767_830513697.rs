plain
.................................................................................................... 9400/11814
.................................................................................................... 9500/11814
.................................................................................................... 9600/11814
.................................i......i........................................................... 9700/11814
...............................................................................iiiiiii..iiiiii.i.... 9800/11814
.................................................................................................... 10000/11814
.................................................................................................... 10100/11814
.................................................................................................... 10200/11814
.................................................................................................... 10300/11814
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.180 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.905 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.46s

 finished in 2.526 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1700/2857
.................................................................................................... 1800/2857
.................................................................................................... 1900/2857
.................................................................................................... 2000/2857
.....................................................................F.F.F.F......F..F.............. 2100/2857
.................................................................................................... 2300/2857
.................................................................................................... 2400/2857
.................................................................................................... 2500/2857
.i...............................................i................i.....................i........... 2600/2857
.i...............................................i................i.....................i........... 2600/2857
..........i.....................i.....................i................................i............ 2700/2857
.........i.....................i.....................i.....................i........................ 2800/2857
.........................................................
failures:

---- src/ops/try_trait.rs - ops::try_trait::Try (line 106) stdout ----
error[E0603]: trait `TryV2` is private
    |
    |
6   | use std::ops::TryV2 as Try;
    |               ^^^^^ private trait
    |
note: the trait `TryV2` is defined here
    |
    |
200 | pub(crate) use self::try_trait::Try as TryV2;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.
For more information about this error, try `rustc --explain E0603`.
Couldn't compile the test.
---- src/ops/try_trait.rs - ops::try_trait::Try (line 42) stdout ----
error[E0603]: trait `TryV2` is private
    |
    |
6   | use std::ops::TryV2 as Try;
    |               ^^^^^ private trait
    |
note: the trait `TryV2` is defined here
    |
    |
200 | pub(crate) use self::try_trait::Try as TryV2;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.
For more information about this error, try `rustc --explain E0603`.
Couldn't compile the test.
---- src/ops/try_trait.rs - ops::try_trait::Try (line 57) stdout ----
error[E0603]: trait `TryV2` is private
    |
    |
7   | use std::ops::{ControlFlow, TryV2 as Try};
    |                             ^^^^^ private trait
    |
note: the trait `TryV2` is defined here
    |
    |
200 | pub(crate) use self::try_trait::Try as TryV2;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.
For more information about this error, try `rustc --explain E0603`.
Couldn't compile the test.
---- src/ops/try_trait.rs - ops::try_trait::Try (line 82) stdout ----
error[E0603]: trait `TryV2` is private
    |
    |
7   | use std::ops::{ControlFlow, TryV2 as Try};
    |                             ^^^^^ private trait
    |
note: the trait `TryV2` is defined here
    |
    |
200 | pub(crate) use self::try_trait::Try as TryV2;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.
For more information about this error, try `rustc --explain E0603`.
Couldn't compile the test.
---- src/ops/try_trait.rs - ops::try_trait::Try::branch (line 208) stdout ----
error[E0603]: trait `TryV2` is private
    |
    |
7   | use std::ops::{ControlFlow, TryV2 as Try};
    |                             ^^^^^ private trait
    |
note: the trait `TryV2` is defined here
    |
    |
200 | pub(crate) use self::try_trait::Try as TryV2;


error[E0599]: no method named `branch` found for enum `Result<{integer}, String>` in the current scope
    |
    |
9   | assert_eq!(Ok::<_, String>(3).branch(), ControlFlow::Continue(3));
    |                               ^^^^^^ method not found in `Result<{integer}, String>`
   ::: /checkout/library/core/src/ops/try_trait.rs:228:8
    |
    |
228 |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
    |        ------ the method is available for `Result<{integer}, String>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
6   | use std::ops::Try;
6   | use std::ops::Try;
    |

error[E0599]: no method named `branch` found for enum `Result<String, {integer}>` in the current scope
    |
    |
10  | assert_eq!(Err::<String, _>(3).branch(), ControlFlow::Break(Err(3)));
    |                                ^^^^^^ method not found in `Result<String, {integer}>`
   ::: /checkout/library/core/src/ops/try_trait.rs:228:8
    |
    |
228 |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
    |        ------ the method is available for `Result<String, {integer}>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
6   | use std::ops::Try;
6   | use std::ops::Try;
    |

error[E0599]: no method named `branch` found for enum `Option<{integer}>` in the current scope
    |
    |
12  | assert_eq!(Some(3).branch(), ControlFlow::Continue(3));
    |                    ^^^^^^ method not found in `Option<{integer}>`
   ::: /checkout/library/core/src/ops/try_trait.rs:228:8
    |
    |
228 |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
    |        ------ the method is available for `Option<{integer}>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
6   | use std::ops::Try;
6   | use std::ops::Try;
    |

error[E0599]: no method named `branch` found for enum `Option<String>` in the current scope
    |
    |
13  | assert_eq!(None::<String>.branch(), ControlFlow::Break(None));
    |                           ^^^^^^ method not found in `Option<String>`
   ::: /checkout/library/core/src/ops/try_trait.rs:228:8
    |
    |
228 |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
    |        ------ the method is available for `Option<String>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
6   | use std::ops::Try;
6   | use std::ops::Try;
    |

error[E0599]: no method named `branch` found for enum `ControlFlow<String, {integer}>` in the current scope
    |
    |
15  | assert_eq!(ControlFlow::<String, _>::Continue(3).branch(), ControlFlow::Continue(3));
    |                                                  ^^^^^^ method not found in `ControlFlow<String, {integer}>`
   ::: /checkout/library/core/src/ops/try_trait.rs:228:8
    |
    |
228 |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
    |        ------ the method is available for `ControlFlow<String, {integer}>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
6   | use std::ops::Try;
6   | use std::ops::Try;
    |

error[E0599]: no method named `branch` found for enum `ControlFlow<{integer}, String>` in the current scope
    |
    |
17  |     ControlFlow::<_, String>::Break(3).branch(),
    |                                        ^^^^^^ method not found in `ControlFlow<{integer}, String>`
   ::: /checkout/library/core/src/ops/try_trait.rs:228:8
    |
    |
228 |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
    |        ------ the method is available for `ControlFlow<{integer}, String>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
6   | use std::ops::Try;
6   | use std::ops::Try;
    |

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0599, E0603.
For more information about an error, try `rustc --explain E0599`.
Couldn't compile the test.
---- src/ops/try_trait.rs - ops::try_trait::Try::from_output (line 175) stdout ----
error[E0603]: trait `TryV2` is private
    |
    |
7   | use std::ops::TryV2 as Try;
    |               ^^^^^ private trait
    |
note: the trait `TryV2` is defined here
    |
    |
200 | pub(crate) use self::try_trait::Try as TryV2;


error[E0599]: no variant or associated item named `from_output` found for enum `Option<_>` in the current scope
   |
   |
17 | assert_eq!(Option::from_output(4)?, 4);
   |                    ^^^^^^^^^^^ variant or associated item not found in `Option<_>`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
6  | use std::ops::Try;
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:52
