plain
........................................................................................ 9504/13260
........................................................................................ 9592/13260
........................................................................................ 9680/13260
........................................................................................ 9768/13260
..............................................................................ii..F..... 9856/13260
........................................................................................ 10032/13260
........................................................................................ 10120/13260
........................................................................................ 10208/13260
........................................................................................ 10296/13260
---
To only update this specific test, also pass `--test-args proc-macro/load-panic-backtrace.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/load-panic-backtrace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/load-panic-backtrace" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "proc-macro-backtrace" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/load-panic-backtrace/auxiliary"
stdout: none
--- stderr -------------------------------
thread '<unnamed>' panicked at 'panic-derive', /checkout/src/test/ui/proc-macro/auxiliary/test-macros.rs:43:5
error: proc-macro derive panicked
  --> /checkout/src/test/ui/proc-macro/load-panic-backtrace.rs:10:10
   |
LL | #[derive(Panic)]
