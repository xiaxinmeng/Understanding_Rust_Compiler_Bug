plain
diff of stderr:

2   --> $DIR/track-diagnostics.rs:LL:CC
3    |
4 LL | const S: A = B;
-    |              ^ expected struct `A`, found struct `B`
+    |              ^ expected `A`, found `B`
6 -Ztrack-diagnostics: created at compiler/rustc_infer/src/infer/error_reporting/mod.rs:LL:CC
8 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/track-diagnostics/track-diagnostics.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args track-diagnostics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/track-diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/track-diagnostics" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/track-diagnostics/auxiliary" "-Z" "track-diagnostics"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/rustdoc-ui/track-diagnostics.rs:10:14
   |
LL | const S: A = B;
LL | const S: A = B;
   |              ^ expected `A`, found `B`
-Ztrack-diagnostics: created at compiler/rustc_infer/src/infer/error_reporting/mod.rs:1922:31
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
