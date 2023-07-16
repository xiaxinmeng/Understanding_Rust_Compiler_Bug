plain
1 error[E0601]: `main` function not found in crate `continue_after_missing_main`
-   --> $DIR/continue-after-missing-main.rs:1:1
+   --> $DIR/continue-after-missing-main.rs:30:2
3    |
- LL | / #![allow(dead_code)]
- LL | |
- LL | | struct Tableau<'a, MP> {
- LL | |     provider: &'a MP,
- LL | |
- LL | | }
-    | |_^ consider adding a `main` function to `$DIR/continue-after-missing-main.rs`
+ LL | }
+ LL | }
+    |  ^ consider adding a `main` function to `$DIR/continue-after-missing-main.rs`
12 
13 error: aborting due to previous error
14 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll/continue-after-missing-main.nll.stderr
To only update this specific test, also pass `--test-args continue-after-missing-main.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/continue-after-missing-main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0601]: `main` function not found in crate `continue_after_missing_main`
  --> /checkout/src/test/ui/continue-after-missing-main.rs:30:2
   |
LL | } //~ ERROR `main` function not found in crate
   |  ^ consider adding a `main` function to `/checkout/src/test/ui/continue-after-missing-main.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.

---
    [ui (nll)] ui/continue-after-missing-main.rs

test result: FAILED. 12417 passed; 1 failed; 161 ignored; 0 measured; 0 filtered out; finished in 80.18s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
