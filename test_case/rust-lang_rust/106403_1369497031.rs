plain

---- [ui] src/test/ui/inference/issue-103587.rs stdout ----
diff of stderr:

26 LL |     if None = x { }
27    |        ^^^^^^^^ expected `bool`, found `()`
- help: you might have meant to use pattern matching
-    |
-    |
- LL |     if let None = x { }
33 help: you might have meant to compare for equality
34    |
34    |
35 LL |     if None == x { }

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-103587/issue-103587.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-103587/issue-103587.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/issue-103587.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-103587.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-103587" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-103587/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected `=`, found `==`
   |
   |
LL |     if let Some(_) == x {}
   |
   |
help: consider using `=` here
   |
LL |     if let Some(_) = x {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/issue-103587.rs:7:8
   |
   |
LL |     if Some(_) = x {}
   |        ^^^^^^^^^^^ expected `bool`, found `()`
help: consider adding `let`
   |
   |
LL |     if let Some(_) = x {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/issue-103587.rs:10:8
   |
   |
LL |     if None = x { }
   |        ^^^^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |     if None == x { }

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
