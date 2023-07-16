plain
To only update this specific test, also pass `--test-args macros/stringify.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/stringify.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/stringify/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/stringify/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `label_break_value` has been stable since 1.64.0 and no longer requires an attribute to enable
   |
LL | #![feature(label_break_value)]
   |            ^^^^^^^^^^^^^^^^^
   |
