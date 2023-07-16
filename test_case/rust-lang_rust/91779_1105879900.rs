plain
ii...................................................................................... 12936/12954
..................
failures:

---- [ui] src/test/ui/feature-gates/feature-gate-debugger-visualizer.rs stdout ----


4 LL | #![debugger_visualizer(natvis_file = "../foo.natvis")]
6    |
+    = note: see issue #95939 <https://github.com/rust-lang/rust/issues/95939> for more information
+    = note: see issue #95939 <https://github.com/rust-lang/rust/issues/95939> for more information
7    = help: add `#![feature(debugger_visualizer)]` to the crate attributes to enable
9 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-debugger-visualizer/feature-gate-debugger-visualizer.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-debugger-visualizer.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-debugger-visualizer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-debugger-visualizer" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-debugger-visualizer/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: the `#[debugger_visualizer]` attribute is an experimental feature
  --> /checkout/src/test/ui/feature-gates/feature-gate-debugger-visualizer.rs:1:1
   |
LL | #![debugger_visualizer(natvis_file = "../foo.natvis")] //~ ERROR the `#[debugger_visualizer]` attribute is an experimental feature
   |
   = note: see issue #95939 <https://github.com/rust-lang/rust/issues/95939> for more information
   = note: see issue #95939 <https://github.com/rust-lang/rust/issues/95939> for more information
   = help: add `#![feature(debugger_visualizer)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
