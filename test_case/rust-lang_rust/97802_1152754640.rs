plain
......................iii............................................................... 13024/13063
.......................................
failures:

---- [ui] src/test/ui/feature-gates/feature-gate-sigpipe_handler.rs stdout ----

2   --> $DIR/feature-gate-sigpipe_handler.rs:3:1
3    |
3    |
4 LL | #![sigpipe_handler(unchanged)]
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: see issue #97889 <https://github.com/rust-lang/rust/issues/97889> for more information
7    = note: see issue #97889 <https://github.com/rust-lang/rust/issues/97889> for more information
8    = help: add `#![feature(sigpipe_handler)]` to the crate attributes to enable

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-sigpipe_handler/feature-gate-sigpipe_handler.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-sigpipe_handler.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-sigpipe_handler.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-sigpipe_handler" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-sigpipe_handler/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: the `#[sigpipe_handler]` attribute is an experimental feature
  --> /checkout/src/test/ui/feature-gates/feature-gate-sigpipe_handler.rs:3:1
   |
LL | #![sigpipe_handler(unchanged)] //~ the `#[sigpipe_handler]` attribute is an experimental feature
   |
   = note: see issue #97889 <https://github.com/rust-lang/rust/issues/97889> for more information
   = note: see issue #97889 <https://github.com/rust-lang/rust/issues/97889> for more information
   = help: add `#![feature(sigpipe_handler)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
