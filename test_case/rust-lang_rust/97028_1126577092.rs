plain
---- [ui] src/test/ui/feature-gates/feature-gate-debugger-visualizer.rs stdout ----
diff of stderr:

7    = note: see issue #95939 <https://github.com/rust-lang/rust/issues/95939> for more information
8    = help: add `#![feature(debugger_visualizer)]` to the crate attributes to enable
- error: aborting due to previous error
- error: aborting due to previous error
+ error: invalid file specified for `#[debugger_visualizer]` attribute.
+    |
+    |
+ LL | #![debugger_visualizer(natvis_file = "../foo.natvis")]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+    = note: visualizer file specified does not exist or could not be read.
+ 
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-debugger-visualizer.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-debugger-visualizer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-debugger-visualizer" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-debugger-visualizer/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: the `#[debugger_visualizer]` attribute is an experimental feature
   |
   |
LL | #![debugger_visualizer(natvis_file = "../foo.natvis")] //~ ERROR the `#[debugger_visualizer]` attribute is an experimental feature
   |
   = note: see issue #95939 <https://github.com/rust-lang/rust/issues/95939> for more information
   = note: see issue #95939 <https://github.com/rust-lang/rust/issues/95939> for more information
   = help: add `#![feature(debugger_visualizer)]` to the crate attributes to enable

error: invalid file specified for `#[debugger_visualizer]` attribute.
   |
   |
LL | #![debugger_visualizer(natvis_file = "../foo.natvis")] //~ ERROR the `#[debugger_visualizer]` attribute is an experimental feature
   |
   = note: visualizer file specified does not exist or could not be read.

error: aborting due to 2 previous errors
