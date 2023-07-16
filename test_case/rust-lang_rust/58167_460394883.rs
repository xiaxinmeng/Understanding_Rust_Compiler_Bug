plain
travis_time:end:15f05658:start=1549307033602728038,finish=1549307035772219093,duration=2169491055
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_codegen-units
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:14] 
[01:07:14] running 39 tests
[01:07:15] i.......i.................FFFFFiFFFFFFF
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-drop-glue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-drop-glue/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-drop-glue" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-drop-glue/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<(u32, local_drop_glue::Struct[0])> @@ local_drop_glue.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Outer[0]> @@ local_drop_glue.3a1fbbbh[Internal]
[01:07:15] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Struct[0]> @@ local_drop_glue.3a1fbbbh[Internal] local_drop_glue.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]> @@ local_drop_glue.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn local_drop_glue::mod1[0]::user[0] @@ local_drop_glue.3a1fbbbh-mod1[External]
[01:07:15] MONO_ITEM fn local_drop_glue::user[0] @@ local_drop_glue.3a1fbbbh[External]
[01:07:15] MONO_ITEM fn local_drop_glue::{{impl}}[0]::drop[0] @@ local_drop_glue.3a1fbbbh[External]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z incremental -Z inline-in-all-cgus -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/extern-drop-glue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-drop-glue/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/extern-drop-glue" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-drop-glue/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<cgu_extern_drop_glue::Struct[0]> @@ extern_drop_glue.3a1fbbbh[Internal] extern_drop_glue.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::LocalStruct[0]> @@ extern_drop_glue.3a1fbbbh[Internal]
[01:07:15] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]> @@ extern_drop_glue.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn extern_drop_glue::mod1[0]::user[0] @@ extern_drop_glue.3a1fbbbh-mod1[External]
[01:07:15] MONO_ITEM fn extern_drop_glue::user[0] @@ extern_drop_glue.3a1fbbbh[External]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z incremental -Z inline-in-all-cgus -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/extern-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zincremental=tmp/partitioning-tests/extern-generic" "-Zshare-generics=y" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn cgu_generic_function::bar[0]<&str> @@ cgu_generic_function.3a1fbbbh-in-extern_generic.3a1fbbbh.volatile[External]
[01:07:15] MONO_ITEM fn cgu_generic_function::foo[0]<&str> @@ cgu_generic_function.3a1fbbbh-in-extern_generic.3a1fbbbh.volatile[External]
[01:07:15] MONO_ITEM fn extern_generic::mod1[0]::mod1[0]::user[0] @@ extern_generic.3a1fbbbh-mod1-mod1[Internal]
[01:07:15] MONO_ITEM fn extern_generic::mod1[0]::user[0] @@ extern_generic.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn extern_generic::mod2[0]::user[0] @@ extern_generic.3a1fbbbh-mod2[Internal]
[01:07:15] MONO_ITEM fn extern_generic::mod3[0]::non_user[0] @@ extern_generic.3a1fbbbh-mod3[Internal]
[01:07:15] MONO_ITEM fn extern_generic::user[0] @@ extern_generic.3a1fbbbh[Internal]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z incremental -Z share-generics=y -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/inlining-from-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/inlining-from-extern-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/inlining-from-extern-crate" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/inlining-from-extern-crate/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn cgu_explicit_inlining::always_inlined[0] @@ inlining_from_extern_crate.3a1fbbbh[Internal] inlining_from_extern_crate.3a1fbbbh-mod2[Internal]
[01:07:15] MONO_ITEM fn cgu_explicit_inlining::inlined[0] @@ inlining_from_extern_crate.3a1fbbbh[Internal] inlining_from_extern_crate.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn inlining_from_extern_crate::mod1[0]::user[0] @@ inlining_from_extern_crate.3a1fbbbh-mod1[External]
[01:07:15] MONO_ITEM fn inlining_from_extern_crate::mod2[0]::user[0] @@ inlining_from_extern_crate.3a1fbbbh-mod2[External]
[01:07:15] MONO_ITEM fn inlining_from_extern_crate::user[0] @@ inlining_from_extern_crate.3a1fbbbh[External]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z incremental -Z inline-in-all-cgus -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-generic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zincremental=tmp/partitioning-tests/local-generic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-generic/auxiliary"
[01:07:15] stdout:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn local_generic::generic[0]<&str> @@ local_generic.3a1fbbbh.volatile[External]
[01:07:15] MONO_ITEM fn local_generic::generic[0]<char> @@ local_generic.3a1fbbbh.volatile[External]
[01:07:15] MONO_ITEM fn local_generic::generic[0]<u32> @@ local_generic.3a1fbbbh.volatile[External]
[01:07:15] MONO_ITEM fn local_generic::generic[0]<u64> @@ local_generic.3a1fbbbh.volatile[External]
[01:07:15] MONO_ITEM fn local_generic::mod1[0]::mod1[0]::user[0] @@ local_generic.3a1fbbbh-mod1-mod1[Internal]
[01:07:15] MONO_ITEM fn local_generic::mod1[0]::user[0] @@ local_generic.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn local_generic::mod2[0]::user[0] @@ local_generic.3a1fbbbh-mod2[Internal]
[01:07:15] MONO_ITEM fn local_generic::user[0] @@ local_generic.3a1fbbbh[Internal]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z incremental -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-inlining-but-not-all.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining-but-not-all/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-inlining-but-not-all" "-Zinline-in-all-cgus=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining-but-not-all/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn local_inlining_but_not_all::inline[0]::inlined_function[0] @@ local_inlining_but_not_all.3a1fbbbh-inline[External]
[01:07:15] MONO_ITEM fn local_inlining_but_not_all::non_user[0]::baz[0] @@ local_inlining_but_not_all.3a1fbbbh-non_user[External]
[01:07:15] MONO_ITEM fn local_inlining_but_not_all::user1[0]::foo[0] @@ local_inlining_but_not_all.3a1fbbbh-user1[External]
[01:07:15] MONO_ITEM fn local_inlining_but_not_all::user2[0]::bar[0] @@ local_inlining_but_not_all.3a1fbbbh-user2[External]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z incremental -Z inline-in-all-cgus=no -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-inlining.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-inlining" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn local_inlining::inline[0]::inlined_function[0] @@ local_inlining.3a1fbbbh-user1[Internal] local_inlining.3a1fbbbh-user2[Internal]
[01:07:15] MONO_ITEM fn local_inlining::non_user[0]::baz[0] @@ local_inlining.3a1fbbbh-non_user[External]
[01:07:15] MONO_ITEM fn local_inlining::user1[0]::foo[0] @@ local_inlining.3a1fbbbh-user1[External]
[01:07:15] MONO_ITEM fn local_inlining::user2[0]::bar[0] @@ local_inlining.3a1fbbbh-user2[External]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z incremental -Z inline-in-all-cgus -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-transitive-inlining.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-transitive-inlining/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-transitive-inlining" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-transitive-inlining/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn local_transitive_inlining::direct_user[0]::foo[0] @@ local_transitive_inlining.3a1fbbbh-indirect_user[Internal]
[01:07:15] MONO_ITEM fn local_transitive_inlining::indirect_user[0]::bar[0] @@ local_transitive_inlining.3a1fbbbh-indirect_user[External]
[01:07:15] MONO_ITEM fn local_transitive_inlining::inline[0]::inlined_function[0] @@ local_transitive_inlining.3a1fbbbh-indirect_user[Internal]
[01:07:15] MONO_ITEM fn local_transitive_inlining::non_user[0]::baz[0] @@ local_transitive_inlining.3a1fbbbh-non_user[External]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z incremental -Z inline-in-all-cgus -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/regular-modules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/regular-modules/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zincremental=tmp/partitioning-tests/regular-modules" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/regular-modules/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn regular_modules::bar[0] @@ regular_modules.3a1fbbbh[Internal]
[01:07:15] MONO_ITEM fn regular_modules::foo[0] @@ regular_modules.3a1fbbbh[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod1[0]::bar[0] @@ regular_modules.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod1[0]::foo[0] @@ regular_modules.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod1[0]::mod1[0]::bar[0] @@ regular_modules.3a1fbbbh-mod1-mod1[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod1[0]::mod1[0]::foo[0] @@ regular_modules.3a1fbbbh-mod1-mod1[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod1[0]::mod2[0]::bar[0] @@ regular_modules.3a1fbbbh-mod1-mod2[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod1[0]::mod2[0]::foo[0] @@ regular_modules.3a1fbbbh-mod1-mod2[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod2[0]::bar[0] @@ regular_modules.3a1fbbbh-mod2[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod2[0]::foo[0] @@ regular_modules.3a1fbbbh-mod2[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod2[0]::mod1[0]::bar[0] @@ regular_modules.3a1fbbbh-mod2-mod1[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod2[0]::mod1[0]::foo[0] @@ regular_modules.3a1fbbbh-mod2-mod1[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod2[0]::mod2[0]::bar[0] @@ regular_modules.3a1fbbbh-mod2-mod2[Internal]
[01:07:15] MONO_ITEM fn regular_modules::mod2[0]::mod2[0]::foo[0] @@ regular_modules.3a1fbbbh-mod2-mod2[Internal]
[01:07:15] MONO_ITEM static regular_modules::BAZ[0] @@ regular_modules.3a1fbbbh[Internal]
[01:07:15] MONO_ITEM static regular_modules::mod1[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM static regular_modules::mod1[0]::mod1[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod1-mod1[Internal]
[01:07:15] MONO_ITEM static regular_modules::mod1[0]::mod2[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod1-mod2[Internal]
[01:07:15] MONO_ITEM static regular_modules::mod2[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod2[Internal]
[01:07:15] MONO_ITEM static regular_modules::mod2[0]::mod1[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod2-mod1[Internal]
[01:07:15] MONO_ITEM static regular_modules::mod2[0]::mod2[0]::BAZ[0] @@ regular_modules.3a1fbbbh-mod2-mod2[Internal]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=eager -Z incremental -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/regular-modules.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/regular-modules.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/statics/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/statics" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/statics/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn statics::function[0] @@ statics.3a1fbbbh[External]
[01:07:15] MONO_ITEM fn statics::mod1[0]::function[0] @@ statics.3a1fbbbh-mod1[External]
[01:07:15] MONO_ITEM static statics::BAR[0] @@ statics.3a1fbbbh[Internal]
[01:07:15] MONO_ITEM static statics::FOO[0] @@ statics.3a1fbbbh[Internal]
[01:07:15] MONO_ITEM static statics::function[0]::BAR[0] @@ statics.3a1fbbbh[Internal]
[01:07:15] MONO_ITEM static statics::function[0]::FOO[0] @@ statics.3a1fbbbh[Internal]
[01:07:15] MONO_ITEM static statics::mod1[0]::BAR[0] @@ statics.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM static statics::mod1[0]::FOO[0] @@ statics.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM static statics::mod1[0]::function[0]::BAR[0] @@ statics.3a1fbbbh-mod1[Internal]
[01:07:15] MONO_ITEM static statics::mod1[0]::function[0]::FOO[0] @@ statics.3a1fbbbh-mod1[Internal]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] warning: static item is never used: `FOO`
[01:07:15] warning: static item is never used: `FOO`
[01:07:15]  --> /checkout/src/test/codegen-units/partitioning/statics.rs:9:1
[01:07:15]   |
[01:07:15] 9 | static FOO: u32 = 0;
[01:07:15]   |
[01:07:15]   = note: #[warn(dead_code)] on by default
[01:07:15] 
[01:07:15] warning: static item is never used: `BAR`
[01:07:15] warning: static item is never used: `BAR`
[01:07:15]   --> /checkout/src/test/codegen-units/partitioning/statics.rs:12:1
[01:07:15]    |
[01:07:15] 12 | static BAR: u32 = 0;
[01:07:15] 
[01:07:15] warning: static item is never used: `FOO`
[01:07:15]   --> /checkout/src/test/codegen-units/partitioning/statics.rs:17:5
[01:07:15]    |
[01:07:15]    |
[01:07:15] 17 |     static FOO: u32 = 0;
[01:07:15] 
[01:07:15] warning: static item is never used: `BAR`
[01:07:15]   --> /checkout/src/test/codegen-units/partitioning/statics.rs:20:5
[01:07:15]    |
[01:07:15]    |
[01:07:15] 20 |     static BAR: u32 = 0;
[01:07:15] 
[01:07:15] warning: static item is never used: `FOO`
[01:07:15]   --> /checkout/src/test/codegen-units/partitioning/statics.rs:25:5
[01:07:15]    |
[01:07:15]    |
[01:07:15] 25 |     static FOO: u32 = 0;
[01:07:15] 
[01:07:15] warning: static item is never used: `BAR`
[01:07:15]   --> /checkout/src/test/codegen-units/partitioning/statics.rs:28:5
[01:07:15]    |
[01:07:15]    |
[01:07:15] 28 |     static BAR: u32 = 0;
[01:07:15] 
[01:07:15] warning: static item is never used: `FOO`
[01:07:15]   --> /checkout/src/test/codegen-units/partitioning/statics.rs:33:9
[01:07:15]    |
[01:07:15]    |
[01:07:15] 33 |         static FOO: u32 = 0;
[01:07:15] 
[01:07:15] warning: static item is never used: `BAR`
[01:07:15]   --> /checkout/src/test/codegen-units/partitioning/statics.rs:36:9
[01:07:15]    |
[01:07:15]    |
[01:07:15] 36 |         static BAR: u32 = 0;
[01:07:15] 
[01:07:15] 
[01:07:15] error: internal compiler error: src/librustc/ty/query/on_disk_cache.rs:949: Encoding DefIndex without context.
[01:07:15] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[01:07:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:15] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:07:15]   left: `LLVMing`,
[01:07:15]   left: `LLVMing`,
[01:07:15]  right: `Codegenning`', src/librustc_codegen_ssa/back/write.rs:1414:21
[01:07:15] 
[01:07:15] 
[01:07:15] note: the compiler unexpectedly panicked. this is a bug.
[01:07:15] 
[01:07:15] 
[01:07:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:15] 
[01:07:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:15] 
[01:07:15] note: compiler flags: -Z threads=1 -Z human_readable_cgu_names -Z unstable-options -Z print-mono-items=lazy -Z incremental -C prefer-dynamic -C rpath
[01:07:15] 
[01:07:15] ------------------------------------------
[01:07:15] 
[01:07:15] thread '[codegen-units] codegen-units/partitioning/statics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] thread '[codegen-units] codegen-units/partitioning/statics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:15] 
[01:07:15] ---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----
[01:07:15] 
[01:07:15] error: compilation failed!
[01:07:15] status: exit code: 101
[01:07:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/vtable-through-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/vtable-through-const/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/vtable-through-const" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/vtable-through-const/auxiliary"
[01:07:15] ------------------------------------------
[01:07:15] ------------------------------------------
[01:07:15] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<u32> @@ vtable_through_const.7rcbfp3g[Internal]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::Trait1[0]::do_something[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::Trait1[0]::do_something_else[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::Trait2[0]::do_something[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::Trait2[0]::do_something_else[0]<u32> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::id[0]<char> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::id[0]<i64> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something_else[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[External]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[01:07:15] MONO_ITEM fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something_else[0]<u8> @@ vtable_through_const.7rcbfp3g-mod1.volatile[Internal]
[01:07:15] MONO_ITEM fn vtable_through_const::start[0] @@ vtable_through_const.7rcbfp3g[Internal]
[01:07:15] ------------------------------------------
[01:07:15] stderr:
[01:07:15] ------------------------------------------
[01:07:15] warning: constant item is never used: `TRAIT2_REF`
[01:07:15] warning: constant item is never used: `TRAIT2_REF`
[01:07:15]   --> /checkout/src/test/codegen-units/partitioning/vtable-through-const.rs:61:5
[01:07:15]    |
---
[01:07:15] test result: FAILED. 24 passed; 12 failed; 3 ignored; 0 measured; 0 filtered out
[01:07:15] 
[01:07:15] 
[01:07:15] 
[01:07:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:15] 
[01:07:15] 
[01:07:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:15] Build completed unsuccessfully in 0:11:02
[01:07:15] Build completed unsuccessfully in 0:11:02
[01:07:15] make: *** [check] Error 1
[01:07:15] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0acdd802
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb  4 20:11:22 UTC 2019
---
travis_time:end:0c744f6f:start=1549311084471973913,finish=1549311084476762455,duration=4788542
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ad4c490
$ ln -s . checkout &&
