
---- [ui] ui/associated-type-bounds/traits-assoc-type-macros.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/home/the8472/workspace/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/the8472/workspace/rust/src/test/ui/associated-type-bounds/traits-assoc-type-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/home/the8472/workspace/rust/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/traits-assoc-type-macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/the8472/workspace/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Clink-arg=-fuse-ld=lld" "-Clink-arg=-Wl,--threads=1" "-Cincremental=tmp/traits-assoc-type-macros" "-L" "/home/the8472/workspace/rust/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/traits-assoc-type-macros/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'try_mark_previous_green() - Forcing the DepNode should have set its color', /home/the8472/workspace/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:597:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C link-arg=-fuse-ld=lld -C link-arg=-Wl,--threads=1 -C incremental

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack

------------------------------------------


---- [ui] ui/async-await/issues/issue-64964.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/home/the8472/workspace/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/the8472/workspace/rust/src/test/ui/async-await/issues/issue-64964.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/home/the8472/workspace/rust/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/the8472/workspace/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Clink-arg=-fuse-ld=lld" "-Clink-arg=-Wl,--threads=1" "-Z" "query-dep-graph" "-C" "incremental=tmp/issue-64964" "--edition=2018" "-L" "/home/the8472/workspace/rust/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at 'try_mark_previous_green() - Forcing the DepNode should have set its color', /home/the8472/workspace/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:597:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -Z query-dep-graph -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C link-arg=-fuse-ld=lld -C link-arg=-Wl,--threads=1 -C incremental

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack

------------------------------------------



failures:
    [ui] ui/associated-type-bounds/traits-assoc-type-macros.rs
    [ui] ui/async-await/issues/issue-64964.rs

