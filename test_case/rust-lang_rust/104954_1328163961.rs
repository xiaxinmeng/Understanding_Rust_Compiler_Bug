plain
........................................................................................ 13904/13915
...........
failures:

---- [ui] src/test/ui/suggestions/seggest_print_over_printf.rs stdout ----

4 LL |     printf("%d", x);
5    |     ^^^^^^ not found in this scope
6    |
6    |
- help: you may want to use the macro `print!(..)`
+ help: you may have meant to use the `print` macro
8    |
9 LL |     print!("%d", x);


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/seggest_print_over_printf/seggest_print_over_printf.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/seggest_print_over_printf.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/seggest_print_over_printf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/seggest_print_over_printf" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/seggest_print_over_printf/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `printf` in this scope
  --> /checkout/src/test/ui/suggestions/seggest_print_over_printf.rs:6:5
LL |     printf("%d", x);
   |     ^^^^^^ not found in this scope
   |
   |
help: you may have meant to use the `print` macro
   |
LL |     print!("%d", x);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
