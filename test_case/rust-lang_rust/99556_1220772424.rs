plain
....iii................................................................................. 13376/13399
.......................
failures:

---- [ui] src/test/ui/feature-gates/feature-gate-collapse_debuginfo.rs stdout ----


4 LL | #[collapse_debuginfo]
6    |
+    = note: see issue #100758 <https://github.com/rust-lang/rust/issues/100758> for more information
+    = note: see issue #100758 <https://github.com/rust-lang/rust/issues/100758> for more information
7    = help: add `#![feature(collapse_debuginfo)]` to the crate attributes to enable
9 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-collapse_debuginfo/feature-gate-collapse_debuginfo.stderr
To only update this specific test, also pass `--test-args feature-gates/feature-gate-collapse_debuginfo.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-collapse_debuginfo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-collapse_debuginfo" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-collapse_debuginfo/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: the `#[collapse_debuginfo]` attribute is an experimental feature
  --> /checkout/src/test/ui/feature-gates/feature-gate-collapse_debuginfo.rs:1:1
   |
LL | #[collapse_debuginfo]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   = note: see issue #100758 <https://github.com/rust-lang/rust/issues/100758> for more information
   = note: see issue #100758 <https://github.com/rust-lang/rust/issues/100758> for more information
   = help: add `#![feature(collapse_debuginfo)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
