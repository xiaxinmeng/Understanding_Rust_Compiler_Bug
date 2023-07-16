plain
........................................................................................ 7568/12930
....i................................................................................... 7656/12930
........................................................................................ 7744/12930
........................................................................................ 7832/12930
ii............F....i......i..ii.................F....................................... 7920/12930
........................................................................................ 8096/12930
........................................................................................ 8184/12930
........................................................................................ 8272/12930
.................................i..ii.................................................. 8360/12930
---
failures:

---- [ui] src/test/ui/modules/dummy.rs stdout ----
normalized stderr:
error[E0601]: `main` function not found in crate `dummy`
   |
LL | pub struct Dummy;
   |                  ^ consider adding a `main` function to `$DIR/dummy.rs`


error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/dummy/dummy.stderr
To only update this specific test, also pass `--test-args modules/dummy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/modules/dummy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/dummy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/dummy/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0601]: `main` function not found in crate `dummy`
   |
LL | pub struct Dummy;
   |                  ^ consider adding a `main` function to `/checkout/src/test/ui/modules/dummy.rs`


error: aborting due to previous error

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
For more information about this error, try `rustc --explain E0601`.
------------------------------------------


---- [ui] src/test/ui/modules/special_module_name_ignore.rs stdout ----
warning: struct is never constructed: `Dummy`
  --> $DIR/dummy.rs:1:12
   |
LL | pub struct Dummy;
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/special_module_name_ignore/special_module_name_ignore.stderr
To only update this specific test, also pass `--test-args modules/special_module_name_ignore.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/modules/special_module_name_ignore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/special_module_name_ignore/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/special_module_name_ignore/auxiliary"
stdout: none
--- stderr -------------------------------
warning: struct is never constructed: `Dummy`
   |
LL | pub struct Dummy;
   |            ^^^^^
   |
