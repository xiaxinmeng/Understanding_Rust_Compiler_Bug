plain

---- compile_test stdout ----
diff of stderr:

-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if !a.is_empty() {
-LL | |         panic!("qaqaq{:?}", a);
-LL | |     }
-   |
-   |
-   = note: `-D clippy::manual-assert` implied by `-D warnings`
-   |
error: test failed, to rerun pass `--test compile-test`
error: test failed, to rerun pass `--test compile-test`
-LL |     assert!(a.is_empty(), "qaqaq{:?}", a);
-   |
+thread 'rustc' panicked at 'Span must not be empty and have no suggestion', /checkout/compiler/rustc_errors/src/diagnostic.rs:652:9
 
 
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if !a.is_empty() {
-LL | |         panic!("qwqwq");
-LL | |     }
-   |
-help: try instead
-   |
-   |
-LL |     assert!(a.is_empty(), "qwqwq");
+error: internal compiler error: unexpected panic
 
 
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if b.is_empty() {
-LL | |         panic!("panic1");
-LL | |     }
-   |
-help: try instead
-   |
-   |
-LL |     assert!(!b.is_empty(), "panic1");
+note: the compiler unexpectedly panicked. this is a bug.
 
 
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if b.is_empty() && a.is_empty() {
-LL | |         panic!("panic2");
-LL | |     }
-   |
-help: try instead
-   |
-   |
-LL |     assert!(!(b.is_empty() && a.is_empty()), "panic2");
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
 
 
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if a.is_empty() && !b.is_empty() {
-LL | |         panic!("panic3");
-LL | |     }
-   |
-help: try instead
-   |
-   |
-LL |     assert!(!(a.is_empty() && !b.is_empty()), "panic3");
-   |
+note: Clippy version: clippy 0.1.66 (1698046e 2022-10-18)
 
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if b.is_empty() || a.is_empty() {
-LL | |         panic!("panic4");
-LL | |     }
-   |
-help: try instead
-   |
-   |
-LL |     assert!(!(b.is_empty() || a.is_empty()), "panic4");
-
-
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if a.is_empty() || !b.is_empty() {
-LL | |         panic!("panic5");
-LL | |     }
-   |
-help: try instead
-   |
-   |
-LL |     assert!(!(a.is_empty() || !b.is_empty()), "panic5");
-
-
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if a.is_empty() {
-LL | |         panic!("with expansion {}", one!())
-LL | |     }
-   |
-help: try instead
-   |
-   |
-LL |     assert!(!a.is_empty(), "with expansion {}", one!());
-
-
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if a > 2 {
-LL | |         // comment
-LL | |         /* this is a
-LL | |         multiline
-...  |
-LL | |         panic!("panic with comment") // comment after `panic!`
-LL | |     }
-   |
-help: try instead
-   |
-   |
-LL |     assert!(!(a > 2), "panic with comment");
-
-error: aborting due to 9 previous errors
-
+query stack during panic:
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/manual_assert.stage-id.edition2018.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/manual_assert.stage-id.edition2018.stderr
thread '[ui] ui/manual_assert.rs' panicked at 'Could not retrieve suggestions from JSON: Error("expected ident", line: 1, column: 2)', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.9.0/src/runtest.rs:2397:15
diff of stderr:

 error: unneeded late initialization
   --> $DIR/needless_late_init.rs:24:5
   --> $DIR/needless_late_init.rs:24:5
    |
 LL |     let a;
    |     ^^^^^^ created here
 LL |     a = "zero";
    |     ^^^^^^^^^^ initialised here
    |
    = note: `-D clippy::needless-late-init` implied by `-D warnings`
 help: declare `a` here
    |
 LL |     let a = "zero";
 
 error: unneeded late initialization
   --> $DIR/needless_late_init.rs:27:5
    |
---
    |     ^^^^^^^^^^^^^ created here
 LL |     d = 1;
    |     ^^^^^ initialised here
    |
 help: declare `d` here
 LL |     let d: usize = 1;
    |     ~~~~~~~~~~~~
 
 error: unneeded late initialization
 error: unneeded late initialization
   --> $DIR/needless_late_init.rs:35:5
    |
 LL |     let e;
    |     ^^^^^^ created here
 LL |     e = format!("{}", d);
    |     ^^^^^^^^^^^^^^^^^^^^ initialised here
    |
 help: declare `e` here
 LL |     let e = format!("{}", d);
    |     ~~~~~
 
-error: unneeded late initialization
---
-LL |     let a = match n {
-   |     +++++++
-help: remove the assignments from the `match` arms
-   |
-LL ~         1 => "one",
-LL |         _ => {
-LL ~             "two"
-   |
-help: add a semicolon after the `match` expression
-LL |     };
-   |      +
-   |      +
+thread 'rustc' panicked at 'Span must not be empty and have no suggestion', /checkout/compiler/rustc_errors/src/diagnostic.rs:570:9
 
-error: unneeded late initialization
-  --> $DIR/needless_late_init.rs:49:5
-   |
-   |
-LL |     let b;
-   |     ^^^^^^
-   |
-help: declare `b` here
-   |
-LL |     let b = if n == 3 {
-help: remove the assignments from the branches
-   |
-   |
-LL ~         "four"
-LL |     } else {
-LL ~         "five"
-help: add a semicolon after the `if` expression
-   |
-LL |     };
-   |      +
---
-   |
-LL |     let d;
-   |     ^^^^^^
-   |
-help: declare `d` here
-LL |     let d = if true {
-   |     +++++++
-help: remove the assignments from the branches
-   |
---
-   |
-LL |     let e;
-   |     ^^^^^^
-   |
-help: declare `e` here
-LL |     let e = if true {
-   |     +++++++
-help: remove the assignments from the branches
-   |
-   |
-LL ~         format!("{} {}", a, b)
-LL ~         format!("{}", n)
-   |
-help: add a semicolon after the `if` expression
-   |
---
-   |
-LL |     let f;
-   |     ^^^^^^
-   |
-help: declare `f` here
-LL |     let f = match 1 {
-   |     +++++++
-help: remove the assignments from the `match` arms
-   |
-   |
-LL -         1 => f = "three",
-LL +         1 => "three",
-   |
+note: Clippy version: clippy 0.1.66 (1698046e 2022-10-18)
-error: unneeded late initialization
-  --> $DIR/needless_late_init.rs:77:5
-   |
-LL |     let g: usize;
-LL |     let g: usize;
-   |     ^^^^^^^^^^^^^
-   |
-help: declare `g` here
-LL |     let g: usize = if true {
-   |     ++++++++++++++
-help: remove the assignments from the branches
-   |
-   |
-LL -         g = 5;
-LL +         5
-help: add a semicolon after the `if` expression
-   |
-LL |     };
-   |      +
-   |      +
-
-error: unneeded late initialization
-  --> $DIR/needless_late_init.rs:85:5
-   |
-LL |     let x;
-   |     ^^^^^^ created here
-LL |     let y = SignificantDrop;
-LL |     x = 1;
-   |     ^^^^^ initialised here
-   |
-help: declare `x` here
-LL |     let x = 1;
-   |     ~~~~~
-
-error: unneeded late initialization
-error: unneeded late initialization
-  --> $DIR/needless_late_init.rs:89:5
-   |
-LL |     let x;
-   |     ^^^^^^ created here
-LL |     let y = 1;
-LL |     x = SignificantDrop;
-   |     ^^^^^^^^^^^^^^^^^^^ initialised here
-   |
-help: declare `x` here
-   |
-LL |     let x = SignificantDrop;
-
-error: unneeded late initialization
-  --> $DIR/needless_late_init.rs:93:5
-   |
-   |
-LL |     let x;
-   |     ^^^^^^ created here
-...
-LL |     x = SignificantDrop;
-   |     ^^^^^^^^^^^^^^^^^^^ initialised here
-   |
-help: declare `x` here
-   |
-LL |     let x = SignificantDrop;
-
-error: unneeded late initialization
-  --> $DIR/needless_late_init.rs:112:5
-   |
---
-LL |     let a = match n {
-   |     +++++++
-help: remove the assignments from the `match` arms
-   |
-LL ~         1 => f().await,
-LL |         _ => {
-LL ~             "two"
-   |
-help: add a semicolon after the `match` expression
-LL |     };
-   |      +
-
-error: unneeded late initialization
---
-LL |     let a = match n {
-   |     +++++++
-help: remove the assignments from the `match` arms
-   |
-LL ~         1 => f(),
-LL |         _ => {
-LL ~             "two"
-   |
-help: add a semicolon after the `match` expression
-LL |     };
-   |      +
-
-error: aborting due to 16 previous errors
-error: aborting due to 16 previous errors
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
+error: aborting due to 5 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_late_init.stage-id.stderr
thread '[ui] ui/needless_late_init.rs' panicked at 'Could not retrieve suggestions from JSON: Error("expected ident", line: 6, column: 2)', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.9.0/src/runtest.rs:2397:15


failures:
    compile_test
