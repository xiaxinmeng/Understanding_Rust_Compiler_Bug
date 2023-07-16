plain
.........................................iii............................................ 14168/14232
................................................................
failures:

---- [ui] tests/ui/dropck/mcp563-dropck-eyepatch-syntax.rs stdout ----
normalized stderr:
error[E0601]: `main` function not found in crate `mcp563_dropck_eyepatch_syntax`
  --> $DIR/mcp563-dropck-eyepatch-syntax.rs:24:2
LL | }
LL | }
   |  ^ consider adding a `main` function to `$DIR/mcp563-dropck-eyepatch-syntax.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/mcp563-dropck-eyepatch-syntax/mcp563-dropck-eyepatch-syntax.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dropck/mcp563-dropck-eyepatch-syntax.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dropck/mcp563-dropck-eyepatch-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/mcp563-dropck-eyepatch-syntax" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/mcp563-dropck-eyepatch-syntax/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error[E0601]: `main` function not found in crate `mcp563_dropck_eyepatch_syntax`
  --> fake-test-src-base/dropck/mcp563-dropck-eyepatch-syntax.rs:24:2
LL | }
LL | }
   |  ^ consider adding a `main` function to `fake-test-src-base/dropck/mcp563-dropck-eyepatch-syntax.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
------------------------------------------
------------------------------------------



failures:
    [ui] tests/ui/dropck/mcp563-dropck-eyepatch-syntax.rs
test result: FAILED. 14095 passed; 1 failed; 136 ignored; 0 measured; 0 filtered out; finished in 142.85s

Build completed unsuccessfully in 0:14:55
