plain
..........................................i.ii.......................................... 14168/14233
.................................................................
failures:

---- [ui] tests/ui/dropck/mcp563-dropck-eyepatch-syntax.rs stdout ----
error: ui test compiled successfully!
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/dropck/mcp563-dropck-eyepatch-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/mcp563-dropck-eyepatch-syntax" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/mcp563-dropck-eyepatch-syntax/auxiliary"
stdout: none
stderr: none


failures:
failures:
    [ui] tests/ui/dropck/mcp563-dropck-eyepatch-syntax.rs
test result: FAILED. 14096 passed; 1 failed; 136 ignored; 0 measured; 0 filtered out; finished in 141.92s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:14:50
