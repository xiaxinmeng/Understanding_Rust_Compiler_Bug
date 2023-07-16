sh
$ ./x.py test src/test/ui/issues/issue-87707.rs --bless --stage 1

~~~snip~~~

running 1 test
F
failures:

---- [ui] ui/issues/issue-87707.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/Users/reez12g/development/rust/build/aarch64-apple-darwin/stage1/bin/rustc" "/Users/reez12g/development/rust/src/test/ui/issues/issue-87707.rs" "-Zthreads=1" "--target=aarch64-apple-darwin" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/reez12g/development/rust/build/aarch64-apple-darwin/test/ui/issues/issue-87707" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/Users/reez12g/development/rust/build/aarch64-apple-darwin/native/rust-test-helpers" "--edition=2018" "-L" "/Users/reez12g/development/rust/build/aarch64-apple-darwin/test/ui/issues/issue-87707/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

------------------------------------------



failures:
    [ui] ui/issues/issue-87707.rs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12657 filtered out; finished in 1.42s

Some tests failed in compiletest suite=ui mode=ui host=aarch64-apple-darwin target=aarch64-apple-darwin
Build completed unsuccessfully in 0:00:08

