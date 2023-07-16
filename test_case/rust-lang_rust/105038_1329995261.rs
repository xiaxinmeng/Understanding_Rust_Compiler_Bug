plain

---- [ui] src/test/ui/suggestions/suggest_print_over_printf.rs stdout ----
diff of stderr:

1 error[E0425]: cannot find function `printf` in this scope
-   --> $DIR/seggest_print_over_printf.rs:5:5
3    |
4 LL |     printf("%d", x);
5    |     ^^^^^^ not found in this scope

---
To only update this specific test, also pass `--test-args suggestions/suggest_print_over_printf.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest_print_over_printf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest_print_over_printf" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest_print_over_printf/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `printf` in this scope
   |
LL |     printf("%d", x);
   |     ^^^^^^ not found in this scope
   |
   |
help: you may have meant to use the `print` macro
   |
LL |     print!("%d", x);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
