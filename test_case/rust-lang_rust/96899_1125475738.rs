plain

---- [ui (nll)] src/test/ui/higher-rank-trait-bounds/issue-59311.rs stdout ----
diff of stderr:

- error[E0477]: the type `&'a V` does not fulfill the required lifetime
+ error: higher-ranked lifetime error
3    |
3    |
4 LL |     v.t(|| {});
5    |     ^^^^^^^^^^
6    |
- note: type must satisfy the static lifetime as required by this binding
-   --> $DIR/issue-59311.rs:15:24
-   --> $DIR/issue-59311.rs:15:24
+    = note: could not prove [closure@$DIR/issue-59311.rs:17:9: 17:14] well-formed
+ error: higher-ranked lifetime error
+   --> $DIR/issue-59311.rs:17:9
9    |
9    |
- LL |     for<'a> &'a V: T + 'static,
-    |                        ^^^^^^^
+ LL |     v.t(|| {});
+    |
+    |
+    = note: could not prove for<'a> &'a V: 'static
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
14 
- For more information about this error, try `rustc --explain E0477`.
- For more information about this error, try `rustc --explain E0477`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-59311.nll/issue-59311.nll.stderr
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/issue-59311.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/issue-59311.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-59311.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-59311.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: higher-ranked lifetime error
   |
   |
LL |     v.t(|| {}); //~ ERROR: `&'a V` does not fulfill the required lifetime
   |
   |
   = note: could not prove [closure@/checkout/src/test/ui/higher-rank-trait-bounds/issue-59311.rs:17:9: 17:14] well-formed
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/higher-rank-trait-bounds/issue-59311.rs:17:9
   |
   |
LL |     v.t(|| {}); //~ ERROR: `&'a V` does not fulfill the required lifetime
   |
   |
   = note: could not prove for<'a> &'a V: 'static
error: aborting due to 2 previous errors
------------------------------------------




failures:
    [ui (nll)] src/test/ui/higher-rank-trait-bounds/issue-59311.rs

test result: FAILED. 12735 passed; 1 failed; 358 ignored; 0 measured; 0 filtered out; finished in 102.80s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
