plain
diff of stderr:

5    |              ^^^^^^^^^^^^^^^^^^
6    |
7    = help: add `#![feature(deprecated_suggestion)]` to the crate root
-    = note: see #XXX for more details
+    = note: see #94785 for more details
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/feature-gate-deprecated_suggestion/feature-gate-deprecated_suggestion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args deprecation/feature-gate-deprecated_suggestion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deprecation/feature-gate-deprecated_suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/feature-gate-deprecated_suggestion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/feature-gate-deprecated_suggestion/auxiliary"
stdout: none
--- stderr -------------------------------
error: suggestions on deprecated items are unstable
  --> /checkout/src/test/ui/deprecation/feature-gate-deprecated_suggestion.rs:5:14
   |
LL | #[deprecated(suggestion = "foo")] //~ ERROR suggestions on deprecated items are unstable
   |
   |
   = help: add `#![feature(deprecated_suggestion)]` to the crate root
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: see #94785 for more details
error: aborting due to previous error
------------------------------------------


