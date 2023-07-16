plain
........................................................................................ 10472/13392
........................................................................................ 10560/13392
..............................................................................i....i..i. 10648/13392
........................................................................................ 10736/13392
...................................i......................................F.F........... 10824/13392
........................................................................................ 11000/13392
........................................................................................ 11088/13392
........................................................................................ 11176/13392
........................................................................................ 11264/13392
---
1 error: unused extern crate
-   --> $DIR/issue-54400-unused-extern-crate-attr-span.rs:12:1
+   --> $DIR/issue-54400-unused-extern-crate-attr-span.rs:11:1
3    |
4 LL | / #[cfg(blandiloquence)]
5 LL | | extern crate edition_lint_paths;
-    | | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
-    | |________________________________|
-    |                                  help: remove it
+    | |________________________________^ help: remove it
---
To only update this specific test, also pass `--test-args rust-2018/issue-54400-unused-extern-crate-attr-span.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "--cfg" "blandiloquence" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs:11:1
   |
   |
LL | / #[cfg(blandiloquence)] //~ HELP remove it
LL | | extern crate edition_lint_paths;
   | |________________________________^ help: remove it
note: the lint level is defined here
  --> /checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs:6:9
   |
LL | #![deny(rust_2018_idioms)]
---
diff of stderr:

15   --> $DIR/remove-extern-crate.rs:32:5
16    |
17 LL |     extern crate core;
-    |     ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
+    |     ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`: `use core;`
20 warning: 2 warnings emitted
21 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/remove-extern-crate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/remove-extern-crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/remove-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "remove_extern_crate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/rust-2018/remove-extern-crate.rs:9:1
   |
   |
LL | extern crate core; //~ WARNING unused extern crate
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/rust-2018/remove-extern-crate.rs:7:9
   |
   |
LL | #![warn(rust_2018_idioms)]
   |         ^^^^^^^^^^^^^^^^
   = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`

warning: `extern crate` is not idiomatic in the new edition
  --> /checkout/src/test/ui/rust-2018/remove-extern-crate.rs:32:5
   |
LL |     extern crate core; //~ WARNING `extern crate` is not idiomatic
   |     ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`: `use core;`
warning: 2 warnings emitted
------------------------------------------


