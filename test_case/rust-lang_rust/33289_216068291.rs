
failures:

---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----

error: compilation failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/test/codegen-units/partitioning/local-transitive-inlining.rs -L x86_64-unknown-linux-gnu/test/codegen-units/ --target=x86_64-unknown-linux-musl -L x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-transitive-inlining.stage2-x86_64-unknown-linux-musl.codegen-units.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-transitive-inlining.stage2-x86_64-unknown-linux-musl -C linker=/musl-x86_64/bin/musl-gcc -C ar=ar --cfg rtopt -C rpath -O -L x86_64-unknown-linux-musl/rt -Zprint-trans-items=lazy -Zincremental=tmp
stdout:
------------------------------------------
TRANS_ITEM fn local_transitive_inlining::direct_user[0]::foo[0] @@ local_transitive_inlining-direct_user[WeakODR] local_transitive_inlining-indirect_user[Available]
TRANS_ITEM fn local_transitive_inlining::indirect_user[0]::bar[0] @@ local_transitive_inlining-indirect_user[WeakODR]
TRANS_ITEM fn local_transitive_inlining::inline[0]::inlined_function[0] @@ local_transitive_inlining-direct_user[Available] local_transitive_inlining-indirect_user[Available] local_transitive_inlining-inline[WeakODR]
TRANS_ITEM fn local_transitive_inlining::non_user[0]::baz[0] @@ local_transitive_inlining-non_user[WeakODR]

------------------------------------------
stderr:
------------------------------------------
error: unable to delete old dep-graph at `tmp/dep_graph.rbml`: No such file or directory (os error 2)
error: aborting due to previous error

------------------------------------------

thread '[codegen-units] codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', /buildslave/rust-buildbot/slave/auto-linux-musl-64-opt/build/src/tools/compiletest/src/runtest.rs:1595
note: Run with `RUST_BACKTRACE=1` for a backtrace.

