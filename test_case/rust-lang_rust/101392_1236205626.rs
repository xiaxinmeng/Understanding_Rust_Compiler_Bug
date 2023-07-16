plain

---- [ui] src/test/ui/feature-gates/feature-gate-profiler-runtime.rs stdout ----
diff of stderr:

- error[E0658]: the `#[profiler_runtime]` attribute is used to identify the `profiler_builtins` crate which contains the profiler runtime and will never be stable
+ error[E0658]: the `#[profiler_runtime]` attribute is used to identify the crate which contains the profiler runtime and will never be stable
3    |
4 LL | #![profiler_runtime]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-profiler-runtime/feature-gate-profiler-runtime.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-profiler-runtime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-profiler-runtime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-profiler-runtime" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-profiler-runtime/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: the `#[profiler_runtime]` attribute is used to identify the crate which contains the profiler runtime and will never be stable
   |
   |
LL | #![profiler_runtime] //~ ERROR the `#[profiler_runtime]` attribute is
   |
   = help: add `#![feature(profiler_runtime)]` to the crate attributes to enable

error: aborting due to previous error
