
$ ./x.py test --bless --no-fail-fast src/test/codegen-units src/test/codegen
Updating only changed submodules
Submodules updated in 0.02 seconds
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.10s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.15s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.10s
Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 tool compiletest (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.10s
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 41 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiFFFiFFFiFFFFFFF
failures:

---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/local-inlining.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/local-inlining" "-Zinline-in-all-cgus" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/local-inlining/local_inlining-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/local-inlining-but-not-all.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining-but-not-all" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/local-inlining-but-not-all" "-Zinline-in-all-cgus=no" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining-but-not-all/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/local-inlining-but-not-all/local_inlining_but_not_all-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus=no -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/local-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-generic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Cincremental=tmp/partitioning-tests/local-generic" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-generic/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/local-generic/local_generic-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/local-drop-glue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-drop-glue" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/local-drop-glue" "-Zinline-in-all-cgus" "-Copt-level=0" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-drop-glue/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/local-drop-glue/local_drop_glue-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental -C opt-level=0

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/incremental-merging.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/incremental-merging.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/incremental-merging" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/incremental-merging" "-Ccodegen-units=3" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/incremental-merging/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/incremental-merging/incremental_merging-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental -C codegen-units=3

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/regular-modules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/regular-modules" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Cincremental=tmp/partitioning-tests/regular-modules" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/regular-modules/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/regular-modules/regular_modules-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/statics" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/statics" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/statics/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/statics/statics-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/vtable-through-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/vtable-through-const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/vtable-through-const" "-Zinline-in-all-cgus" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/vtable-through-const/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/vtable-through-const/vtable_through_const-cvuma7mz0ncd`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/local-transitive-inlining.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-transitive-inlining" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/local-transitive-inlining" "-Zinline-in-all-cgus" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-transitive-inlining/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/local-transitive-inlining/local_transitive_inlining-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/inlining-from-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/inlining-from-extern-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/inlining-from-extern-crate" "-Zinline-in-all-cgus" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/inlining-from-extern-crate/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/inlining-from-extern-crate/inlining_from_extern_crate-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/extern-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Cincremental=tmp/partitioning-tests/extern-generic" "-Zshare-generics=y" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/extern-generic/extern_generic-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z share-generics=y -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/extern-drop-glue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-drop-glue" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Cincremental=tmp/partitioning-tests/extern-drop-glue" "-Zinline-in-all-cgus" "-Copt-level=0" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-drop-glue/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/extern-drop-glue/extern_drop_glue-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z inline-in-all-cgus -C prefer-dynamic -C rpath -C debuginfo=0 -C incremental -C opt-level=0

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------


---- [codegen-units] codegen-units/partitioning/shared-generics.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/simon/src/rust/src/test/codegen-units/partitioning/shared-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "--out-dir" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/shared-generics" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zshare-generics=yes" "-Cincremental=tmp/partitioning-tests/shared-generics-exe" "-Copt-level=0" "-L" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/shared-generics/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/shared-generics-exe/shared_generics-34rilcqhzgbif`: Not a directory (os error 20)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:908:48
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z share-generics=yes -C rpath -C debuginfo=0 -C incremental -C opt-level=0

query stack during panic:
end of query stack
error: aborting due to previous error


------------------------------------------



failures:
    [codegen-units] codegen-units/partitioning/extern-drop-glue.rs
    [codegen-units] codegen-units/partitioning/extern-generic.rs
    [codegen-units] codegen-units/partitioning/incremental-merging.rs
    [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs
    [codegen-units] codegen-units/partitioning/local-drop-glue.rs
    [codegen-units] codegen-units/partitioning/local-generic.rs
    [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs
    [codegen-units] codegen-units/partitioning/local-inlining.rs
    [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs
    [codegen-units] codegen-units/partitioning/regular-modules.rs
    [codegen-units] codegen-units/partitioning/shared-generics.rs
    [codegen-units] codegen-units/partitioning/statics.rs
    [codegen-units] codegen-units/partitioning/vtable-through-const.rs

test result: FAILED. 0 passed; 13 failed; 28 ignored; 0 measured; 0 filtered out; finished in 0.09s

Some tests failed in compiletest suite=codegen-units mode=codegen-units host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/lib" "--run-lib-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--src-base" "/home/simon/src/rust/src/test/codegen-units" "--build-base" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage1-x86_64-unknown-linux-gnu" "--suite" "codegen-units" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--bless" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 1


	finished in 0.138 seconds
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 265 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii.iiiiiiiiiiiiiiiiiii..i.iiiiiiiii.i.iiiiiiiiiiiii 100/265
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii.iiiii...Fiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 200/265
iiii..iF.i.iiiiiiiiiiiii.i.iiiiii.iiiiiiiiiiiiiiiiiiiiiiii.......
failures:

---- [codegen] codegen/async-fn-debug.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen/async-fn-debug/async-fn-debug.ll" "/home/simon/src/rust/src/test/codegen/async-fn-debug.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/simon/src/rust/src/test/codegen/async-fn-debug.rs:24:16: error: CHECK-SAME: is not on the same line as the previous match
// CHECK-SAME: file: [[FILE:![0-9]*]], line: 11,
               ^
/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen/async-fn-debug/async-fn-debug.ll:297:67: note: 'next' match was here
!153 = !DIDerivedType(tag: DW_TAG_member, name: "3", scope: !145, file: !132, line: 11, baseType: !154, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 3)
                                                                  ^
/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen/async-fn-debug/async-fn-debug.ll:291:66: note: previous match ended here
!147 = !DIDerivedType(tag: DW_TAG_member, name: "0", scope: !145, file: !132, line: 10, baseType: !148, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 0)
                                                                 ^

Input file: /home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen/async-fn-debug/async-fn-debug.ll
Check file: /home/simon/src/rust/src/test/codegen/async-fn-debug.rs

-dump-input=help explains the following input dump.

Input was:
<<<<<<
         .
         .
         .
       292: !148 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unresumed", scope: !142, file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !4, templateParams: !4, identifier: "dc267de58dafdcb3dba041b347d5a19b::Unresumed")
       293: !149 = !DIDerivedType(tag: DW_TAG_member, name: "1", scope: !145, file: !132, line: 14, baseType: !150, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 1)
       294: !150 = !DICompositeType(tag: DW_TAG_structure_type, name: "Returned", scope: !142, file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !4, templateParams: !4, identifier: "dc267de58dafdcb3dba041b347d5a19b::Returned")
       295: !151 = !DIDerivedType(tag: DW_TAG_member, name: "2", scope: !145, file: !132, line: 14, baseType: !152, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 2)
       296: !152 = !DICompositeType(tag: DW_TAG_structure_type, name: "Panicked", scope: !142, file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !4, templateParams: !4, identifier: "dc267de58dafdcb3dba041b347d5a19b::Panicked")
       297: !153 = !DIDerivedType(tag: DW_TAG_member, name: "3", scope: !145, file: !132, line: 11, baseType: !154, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 3)
same:24                                                                       !~~~~~~~~~~~~~~~~~~~~                                                                                  error: match on wrong line
       298: !154 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend0", scope: !142, file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !155, templateParams: !4, identifier: "dc267de58dafdcb3dba041b347d5a19b::Suspend0")
       299: !155 = !{!156}
       300: !156 = !DIDerivedType(tag: DW_TAG_member, name: "pinned", scope: !154, file: !2, baseType: !157, size: 8, align: 8, offset: 8)
       301: !157 = !DICompositeType(tag: DW_TAG_structure_type, name: "GenFuture<generator-0>", scope: !138, file: !2, size: 8, align: 8, elements: !158, templateParams: !172, identifier: "c7ea9509a40130cbdd3a2b95c8c6cb81")
       302: !158 = !{!159}
         .
         .
         .
>>>>>>

------------------------------------------


---- [codegen] codegen/generator-debug.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll" "/home/simon/src/rust/src/test/codegen/generator-debug.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/simon/src/rust/src/test/codegen/generator-debug.rs:28:16: error: CHECK-SAME: is not on the same line as the previous match
// CHECK-SAME: file: [[FILE:![0-9]*]], line: 14,
               ^
/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll:292:67: note: 'next' match was here
!148 = !DIDerivedType(tag: DW_TAG_member, name: "3", scope: !140, file: !132, line: 14, baseType: !149, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 3)
                                                                  ^
/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll:286:66: note: previous match ended here
!142 = !DIDerivedType(tag: DW_TAG_member, name: "0", scope: !140, file: !132, line: 13, baseType: !143, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 0)
                                                                 ^

Input file: /home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll
Check file: /home/simon/src/rust/src/test/codegen/generator-debug.rs

-dump-input=help explains the following input dump.

Input was:
<<<<<<
         .
         .
         .
       287: !143 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unresumed", scope: !137, file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !4, templateParams: !4, identifier: "3f0c4f80d3c56b3bc0980e3ddc0bdfac::Unresumed")
       288: !144 = !DIDerivedType(tag: DW_TAG_member, name: "1", scope: !140, file: !132, line: 17, baseType: !145, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 1)
       289: !145 = !DICompositeType(tag: DW_TAG_structure_type, name: "Returned", scope: !137, file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !4, templateParams: !4, identifier: "3f0c4f80d3c56b3bc0980e3ddc0bdfac::Returned")
       290: !146 = !DIDerivedType(tag: DW_TAG_member, name: "2", scope: !140, file: !132, line: 17, baseType: !147, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 2)
       291: !147 = !DICompositeType(tag: DW_TAG_structure_type, name: "Panicked", scope: !137, file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !4, templateParams: !4, identifier: "3f0c4f80d3c56b3bc0980e3ddc0bdfac::Panicked")
       292: !148 = !DIDerivedType(tag: DW_TAG_member, name: "3", scope: !140, file: !132, line: 14, baseType: !149, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 3)
same:28                                                                       !~~~~~~~~~~~~~~~~~~~~                                                                                  error: match on wrong line
       293: !149 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend0", scope: !137, file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !4, templateParams: !4, identifier: "3f0c4f80d3c56b3bc0980e3ddc0bdfac::Suspend0")
       294: !150 = !DIDerivedType(tag: DW_TAG_member, name: "4", scope: !140, file: !132, line: 16, baseType: !151, size: 256, align: 64, flags: DIFlagArtificial, extraData: i64 4)
       295: !151 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend1", scope: !137, file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !152, templateParams: !4, identifier: "3f0c4f80d3c56b3bc0980e3ddc0bdfac::Suspend1")
       296: !152 = !{!153}
       297: !153 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !151, file: !2, baseType: !154, size: 192, align: 64)
         .
         .
         .
>>>>>>

------------------------------------------



failures:
    [codegen] codegen/async-fn-debug.rs
    [codegen] codegen/generator-debug.rs

test result: FAILED. 24 passed; 2 failed; 239 ignored; 0 measured; 0 filtered out; finished in 0.28s

Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/lib" "--run-lib-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--src-base" "/home/simon/src/rust/src/test/codegen" "--build-base" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage1-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--bless" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 1


	finished in 0.337 seconds

2 command(s) did not execute successfully:

  - "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/lib" "--run-lib-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--src-base" "/home/simon/src/rust/src/test/codegen-units" "--build-base" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage1-x86_64-unknown-linux-gnu" "--suite" "codegen-units" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--bless" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""

  - "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/lib" "--run-lib-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--src-base" "/home/simon/src/rust/src/test/codegen" "--build-base" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage1-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/simon/src/rust/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--bless" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/simon/src/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""

failed to run: /home/simon/src/rust/build/bootstrap/debug/bootstrap test --bless --no-fail-fast src/test/codegen-units src/test/codegen
Build completed unsuccessfully in 0:00:01
