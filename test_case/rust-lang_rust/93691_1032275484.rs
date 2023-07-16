plain
---- [ui] ui/impl-trait/issues/issue-78722.rs stdout ----
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
diff of stderr:

1 error[E0658]: `async` blocks are not allowed in constants
-   --> $DIR/issue-78722.rs:12:20
3    |
3    |
4 LL |         let f: F = async { 1 };


8    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
10 error[E0493]: destructors cannot be evaluated at compile-time
-   --> $DIR/issue-78722.rs:12:13
+   --> $DIR/issue-78722.rs:13:13
12    |
12    |
13 LL |         let f: F = async { 1 };
14    |             ^ constants cannot evaluate destructors

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722/issue-78722.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/issues/issue-78722.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-78722.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `async` blocks are not allowed in constants
   |
   |
LL |         let f: F = async { 1 };
   |
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/impl-trait/issues/issue-78722.rs:13:13
   |
   |
LL |         let f: F = async { 1 };
   |             ^ constants cannot evaluate destructors
LL |     }],
   |     - value is dropped here


error[E0271]: type mismatch resolving `<impl Future<Output = [async output]> as Future>::Output == u8`
   |
LL |             async {}
   |             ^^^^^^^^ expected `()`, found `u8`

