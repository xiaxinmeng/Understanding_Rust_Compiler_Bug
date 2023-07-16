
failures:

---- [ui] ui/impl-trait/xcrate.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/joshua/rustc/src/test/ui/impl-trait/xcrate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/xcrate/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/joshua/rustc/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/xcrate/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:802:9: cannot create local mono-item for DefId(18:3 ~ xcrate[8787]::some_internal_fn[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:915:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.47.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: aborting due to previous error


------------------------------------------


---- [ui] ui/impl-trait/xcrate_simple.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/joshua/rustc/src/test/ui/impl-trait/xcrate_simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/xcrate_simple/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/joshua/rustc/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/xcrate_simple/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:802:9: cannot create local mono-item for DefId(18:4 ~ xcrate[8787]::other_internal_fn[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:915:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.47.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: aborting due to previous error


------------------------------------------


---- [ui] ui/issues/issue-50865-private-impl-trait/main.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/joshua/rustc/src/test/ui/issues/issue-50865-private-impl-trait/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50865-private-impl-trait/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/joshua/rustc/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50865-private-impl-trait/main/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:802:9: cannot create local mono-item for DefId(18:7 ~ lib[8787]::foo[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:915:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.47.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: aborting due to previous error


------------------------------------------


---- [ui] ui/generator/xcrate-reachable.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/joshua/rustc/src/test/ui/generator/xcrate-reachable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/ui/generator/xcrate-reachable/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/joshua/rustc/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/ui/generator/xcrate-reachable/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:802:9: cannot create local mono-item for DefId(18:4 ~ xcrate_reachable[8787]::msg[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:915:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.47.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

error: aborting due to previous error


------------------------------------------



failures:
    [ui] ui/generator/xcrate-reachable.rs
    [ui] ui/impl-trait/xcrate.rs
    [ui] ui/impl-trait/xcrate_simple.rs
    [ui] ui/issues/issue-50865-private-impl-trait/main.rs

test result: FAILED. 3 passed; 4 failed; 10535 ignored; 0 measured; 0 filtered out
