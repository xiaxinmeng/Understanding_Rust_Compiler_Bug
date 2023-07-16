plain
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 172 tests
...................................................i.................................... 88/172
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....F...............F...............................................................

---- [ui] src/test/rustdoc-ui/intra-doc/unresolved-import-recovery.rs stdout ----
diff of stderr:


3    |
4 LL | use unresolved_crate::module::Name;
5    |     ^^^^^^^^^^^^^^^^ maybe a missing crate `unresolved_crate`?
+    |
+    = help: consider adding `extern crate unresolved_crate` to use the `unresolved_crate` crate
7 error: Compilation failed, aborting rustdoc
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/unresolved-import-recovery/unresolved-import-recovery.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intra-doc/unresolved-import-recovery.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/unresolved-import-recovery.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/unresolved-import-recovery" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/unresolved-import-recovery/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: maybe a missing crate `unresolved_crate`?
   |
   |
LL | use unresolved_crate::module::Name; //~ ERROR failed to resolve
   |     ^^^^^^^^^^^^^^^^ maybe a missing crate `unresolved_crate`?
   |
   = help: consider adding `extern crate unresolved_crate` to use the `unresolved_crate` crate
error: Compilation failed, aborting rustdoc

error: aborting due to 2 previous errors

---
---- [ui] src/test/rustdoc-ui/issue-61732.rs stdout ----
diff of stderr:

3    |
4 LL | pub(in crate::r#mod) fn main() {}
5    |               ^^^^^ maybe a missing crate `r#mod`?
+    |
+    = help: consider adding `extern crate r#mod` to use the `r#mod` crate
7 error: Compilation failed, aborting rustdoc
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-61732/issue-61732.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-61732.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-61732.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-61732" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-61732/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: maybe a missing crate `r#mod`?
   |
   |
LL | pub(in crate::r#mod) fn main() {}
   |               ^^^^^ maybe a missing crate `r#mod`?
   |
   = help: consider adding `extern crate r#mod` to use the `r#mod` crate
error: Compilation failed, aborting rustdoc

error: aborting due to 2 previous errors

