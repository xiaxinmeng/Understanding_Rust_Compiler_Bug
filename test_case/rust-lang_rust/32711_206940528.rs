


---- [compile-fail] compile-fail/symbol-names/issue-32709.rs stdout ----

error: unexpected compiler message: '/buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/compile-fail/symbol-names/issue-32709.rs:16:5: 16:6 error: unresolved name `a` [E0425]'

error: unexpected compiler message: '/buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/compile-fail/symbol-names/issue-32709.rs:16:5: 16:7 error: mismatched types:'

error: expected error on line 16 not found: 2:5: 2:7

error: 2 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/compile-fail/symbol-names/issue-32709.rs -L x86_64-unknown-linux-gnu/test/compile-fail/ --target=x86_64-unknown-linux-musl -L x86_64-unknown-linux-gnu/test/compile-fail/symbol-names/issue-32709.stage2-x86_64-unknown-linux-musl.compile-fail.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/compile-fail/symbol-names/issue-32709.stage2-x86_64-unknown-linux-musl -C linker=/musl-x86_64/bin/musl-gcc -C ar=ar --cfg rtopt -C rpath -O -L x86_64-unknown-linux-musl/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/compile-fail/symbol-names/issue-32709.rs:16:5: 16:6 error: unresolved name `a` [E0425]
/buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/compile-fail/symbol-names/issue-32709.rs:16     a?; //~ ERROR 2:5: 2:7
                                                                                                                      ^
/buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/compile-fail/symbol-names/issue-32709.rs:16:5: 16:6 help: run `rustc --explain E0425` to see a detailed explanation
/buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/compile-fail/symbol-names/issue-32709.rs:16:5: 16:7 error: mismatched types:
 expected `()`,
    found `std::result::Result<_, _>`
(expected (),
    found enum `std::result::Result`) [E0308]
/buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/compile-fail/symbol-names/issue-32709.rs:16     a?; //~ ERROR 2:5: 2:7
                                                                                                                      ^~
/buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/compile-fail/symbol-names/issue-32709.rs:16:5: 16:7 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error

------------------------------------------

thread '[compile-fail] compile-fail/symbol-names/issue-32709.rs' panicked at 'explicit panic', /buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/compiletest/runtest.rs:1677


failures:
    [compile-fail] compile-fail/symbol-names/issue-32709.rs
