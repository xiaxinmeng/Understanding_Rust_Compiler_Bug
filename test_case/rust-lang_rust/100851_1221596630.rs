plain
diff of stderr:

10   --> $DIR/format-args-capture-issue-93378.rs:9:23
11    |
12 LL |     println!("{a:.n$} {b:.*}");
-    |                   -   ^^^--^
-    |                   |      |
-    |                   |      this precision flag adds an extra required argument at position 0, which is why there are 3 arguments expected
-    |                   this parameter corresponds to the precision flag
+    |                  ---  ^^^--^
+    |                  ||      |
+    |                  ||      this precision flag adds an extra required argument at position 0, which is why there are 4 arguments expected
+    |                  |this parameter corresponds to the precision flag
+    |                  |this parameter corresponds to the precision flag
+    |                  this precision flag adds an extra required argument at position 0, which is why there are 4 arguments expected
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
18    = note: positional arguments are zero-based
18    = note: positional arguments are zero-based
19    = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-issue-93378/format-args-capture-issue-93378.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/format-args-capture-issue-93378.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-args-capture-issue-93378.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-issue-93378" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture-issue-93378/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid reference to positional arguments 1 and 2 (there is 1 argument)
   |
   |
LL |     println!("{a} {b} {} {} {c} {}", c = "c");
   |                          ^^     ^^
   = note: positional arguments are zero-based


error: invalid reference to positional argument 0 (no arguments were given)
   |
   |
LL |     println!("{a:.n$} {b:.*}");
   |                  ---  ^^^--^
   |                  ||      |
   |                  ||      this precision flag adds an extra required argument at position 0, which is why there are 4 arguments expected
   |                  |this parameter corresponds to the precision flag
   |                  |this parameter corresponds to the precision flag
   |                  this precision flag adds an extra required argument at position 0, which is why there are 4 arguments expected
   = note: positional arguments are zero-based
   = note: positional arguments are zero-based
   = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
error: aborting due to 2 previous errors
------------------------------------------


