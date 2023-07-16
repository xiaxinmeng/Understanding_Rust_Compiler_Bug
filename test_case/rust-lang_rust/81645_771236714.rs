plain
.................................................................................................... 7400/11332
............................................................................................i..ii... 7500/11332
............................................................ii...................................... 7600/11332
................................................................................i................... 7700/11332
................................................................i..................F.......FF....... 7800/11332
.....................F..................................i........................................... 7900/11332
.................................................................................................... 8100/11332
...................................i................................................................ 8200/11332
.................................................................................................... 8300/11332
....................................................i............................................... 8400/11332
---
51 
+ warning: panic message is not a string literal
+   --> $DIR/const_panic.rs:18:27
+    |
+ LL | const W: () = std::panic!(MSG);
+    |
+    |
+    = note: `#[warn(non_fmt_panic)]` on by default
+    = note: this is no longer accepted in Rust 2021
+ help: add a "{}" format string to Display the message
+    |
+ LL | const W: () = std::panic!("{}", MSG);
+    |                           ^^^^^
+ help: or use std::panic::panic_any instead
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL | const W: () = std::panic::panic_any(MSG);
+ 
+ 
52 error: any use of this value will cause an error
54    |

99    |
100    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
100    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
101 
- error: aborting due to 10 previous errors
+ warning: panic message is not a string literal
+   --> $DIR/const_panic.rs:33:33
+    |
+ LL | const W_CORE: () = core::panic!(MSG);
+    |
+    = note: this is no longer accepted in Rust 2021
+    = note: this is no longer accepted in Rust 2021
+ help: add a "{}" format string to Display the message
+    |
+ LL | const W_CORE: () = core::panic!("{}", MSG);
+ 
+ error: aborting due to 10 previous errors; 2 warnings emitted
103 
104 
104 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/const_panic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const_panic.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL | const Z: () = std::panic!("cheese");
   |               |
   |               |
   |               the evaluated program panicked at 'cheese', /checkout/src/test/ui/consts/const-eval/const_panic.rs:6:15
   |
   = note: `#[deny(const_err)]` on by default
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: any use of this value will cause an error
   |
   |
LL | const Z2: () = std::panic!();
   |                |
   |                the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/const-eval/const_panic.rs:9:16
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: any use of this value will cause an error
   |
   |
LL | const Y: () = std::unreachable!();
   |               |
   |               |
   |               the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic.rs:12:15
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: any use of this value will cause an error
   |
   |
LL | const X: () = std::unimplemented!();
   |               |
   |               |
   |               the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic.rs:15:15
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: any use of this value will cause an error
   |
   |
LL | const W: () = std::panic!(MSG);
   |               |
   |               |
   |               the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic.rs:18:15
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

warning: panic message is not a string literal
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:18:27
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:18:27
   |
LL | const W: () = std::panic!(MSG);
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL | const W: () = std::panic!("{}", MSG);
   |                           ^^^^^
help: or use std::panic::panic_any instead
   |
LL | const W: () = std::panic::panic_any(MSG);


error: any use of this value will cause an error
   |
   |
LL | const Z_CORE: () = core::panic!("cheese");
   |                    |
   |                    |
   |                    the evaluated program panicked at 'cheese', /checkout/src/test/ui/consts/const-eval/const_panic.rs:21:20
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: any use of this value will cause an error
   |
   |
LL | const Z2_CORE: () = core::panic!();
   |                     |
   |                     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/const-eval/const_panic.rs:24:21
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: any use of this value will cause an error
   |
   |
LL | const Y_CORE: () = core::unreachable!();
   |                    |
   |                    |
   |                    the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic.rs:27:20
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: any use of this value will cause an error
   |
   |
LL | const X_CORE: () = core::unimplemented!();
   |                    |
   |                    |
   |                    the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic.rs:30:20
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: any use of this value will cause an error
   |
   |
LL | const W_CORE: () = core::panic!(MSG);
   |                    |
   |                    |
   |                    the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic.rs:33:20
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

warning: panic message is not a string literal
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:33:33
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:33:33
   |
LL | const W_CORE: () = core::panic!(MSG);
   |
   = note: this is no longer accepted in Rust 2021
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL | const W_CORE: () = core::panic!("{}", MSG);

error: aborting due to 10 previous errors; 2 warnings emitted


---
normalized stderr:
warning: panic message is not a string literal
  --> $DIR/dynamic-drop.rs:49:20
   |
LL |             panic!(InjectedFailure);
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |             panic!("{}", InjectedFailure);
   |                    ^^^^^
help: or use std::panic::panic_any instead
   |
LL |             std::panic::panic_any(InjectedFailure);

warning: panic message is not a string literal
  --> $DIR/dynamic-drop.rs:70:20
   |
   |
LL |             panic!(InjectedFailure);
   |
   = note: this is no longer accepted in Rust 2021
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |             panic!("{}", InjectedFailure);
   |                    ^^^^^
help: or use std::panic::panic_any instead
   |
LL |             std::panic::panic_any(InjectedFailure);

warning: 2 warnings emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/dynamic-drop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args drop/dynamic-drop.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/dynamic-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: panic message is not a string literal
  --> /checkout/src/test/ui/drop/dynamic-drop.rs:49:20
   |
LL |             panic!(InjectedFailure);
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |             panic!("{}", InjectedFailure);
   |                    ^^^^^
help: or use std::panic::panic_any instead
   |
LL |             std::panic::panic_any(InjectedFailure);

warning: panic message is not a string literal
  --> /checkout/src/test/ui/drop/dynamic-drop.rs:70:20
   |
   |
LL |             panic!(InjectedFailure);
   |
   = note: this is no longer accepted in Rust 2021
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |             panic!("{}", InjectedFailure);
   |                    ^^^^^
help: or use std::panic::panic_any instead
   |
LL |             std::panic::panic_any(InjectedFailure);

warning: 2 warnings emitted


---
normalized stderr:
warning: panic message is not a string literal
  --> $DIR/dynamic-drop-async.rs:85:20
   |
LL |             panic!(InjectedFailure);
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |             panic!("{}", InjectedFailure);
   |                    ^^^^^
help: or use std::panic::panic_any instead
   |
LL |             std::panic::panic_any(InjectedFailure);

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/dynamic-drop-async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args drop/dynamic-drop-async.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/dynamic-drop-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: panic message is not a string literal
  --> /checkout/src/test/ui/drop/dynamic-drop-async.rs:85:20
   |
LL |             panic!(InjectedFailure);
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |             panic!("{}", InjectedFailure);
   |                    ^^^^^
help: or use std::panic::panic_any instead
   |
LL |             std::panic::panic_any(InjectedFailure);

warning: 1 warning emitted


---
normalized stderr:
warning: panic message is not a string literal
  --> $DIR/assert-macro-owned.rs:6:20
   |
LL |     assert!(false, "test-assert-owned".to_string());
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |     assert!(false, "{}", "test-assert-owned".to_string());

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-owned/assert-macro-owned.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/assert-macro-owned.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/assert-macro-owned.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-owned/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-owned/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: panic message is not a string literal
  --> /checkout/src/test/ui/macros/assert-macro-owned.rs:6:20
   |
LL |     assert!(false, "test-assert-owned".to_string());
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |     assert!(false, "{}", "test-assert-owned".to_string());

warning: 1 warning emitted


---
normalized stderr:
warning: panic message is not a string literal
  --> $DIR/mir_drop_order.rs:41:43
   |
LL |         (d(4), &d(5), d(6), &d(7), panic!(InjectedFailure));
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |         (d(4), &d(5), d(6), &d(7), panic!("{}", InjectedFailure));
   |                                           ^^^^^
help: or use std::panic::panic_any instead
   |
LL |         (d(4), &d(5), d(6), &d(7), std::panic::panic_any(InjectedFailure));

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_drop_order/mir_drop_order.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mir/mir_drop_order.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/mir_drop_order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_drop_order/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_drop_order/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: panic message is not a string literal
  --> /checkout/src/test/ui/mir/mir_drop_order.rs:41:43
   |
LL |         (d(4), &d(5), d(6), &d(7), panic!(InjectedFailure));
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |         (d(4), &d(5), d(6), &d(7), panic!("{}", InjectedFailure));
   |                                           ^^^^^
help: or use std::panic::panic_any instead
   |
LL |         (d(4), &d(5), d(6), &d(7), std::panic::panic_any(InjectedFailure));

warning: 1 warning emitted


---
normalized stderr:
warning: panic message is not a string literal
  --> $DIR/explicit-panic-msg.rs:13:12
   |
LL |     panic!(format!("woooo{}", "o"));
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/explicit-panic-msg/explicit-panic-msg.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panics/explicit-panic-msg.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/explicit-panic-msg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/explicit-panic-msg/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/explicit-panic-msg/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: panic message is not a string literal
  --> /checkout/src/test/ui/panics/explicit-panic-msg.rs:13:12
   |
LL |     panic!(format!("woooo{}", "o"));
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021

warning: 1 warning emitted


---
normalized stderr:
warning: panic message is not a string literal
  --> $DIR/panic-macro-any-wrapped.rs:6:12
   |
LL |     panic!(Box::new(612_i64));
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |     panic!("{}", Box::new(612_i64));
   |            ^^^^^
help: or use std::panic::panic_any instead
   |
LL |     std::panic::panic_any(Box::new(612_i64));

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any-wrapped/panic-macro-any-wrapped.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panics/panic-macro-any-wrapped.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/panic-macro-any-wrapped.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any-wrapped/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any-wrapped/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: panic message is not a string literal
  --> /checkout/src/test/ui/panics/panic-macro-any-wrapped.rs:6:12
   |
LL |     panic!(Box::new(612_i64));
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |     panic!("{}", Box::new(612_i64));
   |            ^^^^^
help: or use std::panic::panic_any instead
   |
LL |     std::panic::panic_any(Box::new(612_i64));

warning: 1 warning emitted


---
normalized stderr:
warning: panic message is not a string literal
  --> $DIR/panic-macro-any.rs:8:12
   |
LL |     panic!(box 413 as Box<dyn std::any::Any + Send>);
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |     panic!("{}", box 413 as Box<dyn std::any::Any + Send>);
   |            ^^^^^
help: or use std::panic::panic_any instead
   |
LL |     std::panic::panic_any(box 413 as Box<dyn std::any::Any + Send>);

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/panic-macro-any.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panics/panic-macro-any.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/panic-macro-any.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: panic message is not a string literal
  --> /checkout/src/test/ui/panics/panic-macro-any.rs:8:12
   |
LL |     panic!(box 413 as Box<dyn std::any::Any + Send>);
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |     panic!("{}", box 413 as Box<dyn std::any::Any + Send>);
   |            ^^^^^
help: or use std::panic::panic_any instead
   |
LL |     std::panic::panic_any(box 413 as Box<dyn std::any::Any + Send>);

warning: 1 warning emitted


---
  --> $DIR/while-panic.rs:8:12
   |
LL |       panic!({
   |  ____________^
LL | |         while true {
LL | |             panic!("giraffe")
LL | |         }
LL | |         "clandestine"
LL | |     });
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |     panic!("{}", {
   |            ^^^^^
help: or use std::panic::panic_any instead
LL |     std::panic::panic_any({
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: 1 warning emitted
warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/while-panic/while-panic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panics/while-panic.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/while-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/while-panic/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/while-panic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: panic message is not a string literal
  --> /checkout/src/test/ui/panics/while-panic.rs:8:12
   |
LL |       panic!({
   |  ____________^
LL | |         while true {
LL | |             panic!("giraffe")
LL | |         }
LL | |         "clandestine"
LL | |     });
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |     panic!("{}", {
   |            ^^^^^
help: or use std::panic::panic_any instead
LL |     std::panic::panic_any({
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: 1 warning emitted
---
test result: FAILED. 11235 passed; 9 failed; 88 ignored; 0 measured; 0 filtered out; finished in 137.44s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:20
