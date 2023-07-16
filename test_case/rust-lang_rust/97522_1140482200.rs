plain
To only update this specific test, also pass `--test-args consts/const-eval/issue-91827-extern-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-91827-extern-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-91827-extern-types/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-91827-extern-types/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `const_slice_from_raw_parts` has been stable since 1.63.0 and no longer requires an attribute to enable
   |
LL | #![feature(const_slice_from_raw_parts)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args consts/const-size_of_val-align_of_val.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of_val-align_of_val.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of_val-align_of_val/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of_val-align_of_val/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `const_slice_from_raw_parts` has been stable since 1.63.0 and no longer requires an attribute to enable
   |
LL | #![feature(const_slice_from_raw_parts)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
