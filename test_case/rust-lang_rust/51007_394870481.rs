plain
[00:51:10] i..............................................................................i....................
[00:51:15] ....................................................................................................
[00:51:21] ....................................................................................................
[00:51:27] ....................................................................................................
[00:51:32] ............i..................iiiiiiiii..................................................
[00:51:32] 
[00:51:32] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:52:23] i..............................................................................i....................
[00:52:28] ....................................................................................................
[00:52:33] ....................................................................................................
[00:52:39] ....................................................................................................
[00:52:43] ............i.................iiiiiiiii...................................................
[00:52:43] 
[00:52:43]  finished in 71.494
[00:52:43] travis_fold:end:test_ui_nll

---
travis_time:start:test_codegen-units
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:18] 
[01:05:18] running 39 tests
[01:05:21] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:05:21] i.......i.................FFFFFiFFFFFF.
[01:05:21] 
[01:05:21] ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/extern-drop-glue.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-drop-glue/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/extern-drop-glue" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-drop-glue/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<cgu_extern_drop_glue::Struct[0]> @@ extern_drop_glue[Internal] extern_drop_glue-mod1[Internal]
[01:05:21] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::LocalStruct[0]> @@ extern_drop_glue[Internal]
[01:05:21] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]> @@ extern_drop_glue-mod1[Internal]
[01:05:21] MONO_ITEM fn extern_drop_glue::mod1[0]::user[0] @@ extern_drop_glue-mod1[External]
[01:05:21] MONO_ITEM fn extern_drop_glue::user[0] @@ extern_drop_glue[External]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] 
[01:05:21] error: aborting due to previous error
[01:05:21] 
---
[01:05:21] ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-drop-glue.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-drop-glue/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-drop-glue" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-drop-glue/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(u32, local_drop_glue::Struct[0])> @@ local_drop_glue-mod1[Internal]
[01:05:21] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Outer[0]> @@ local_drop_glue[Internal]
[01:05:21] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Struct[0]> @@ local_drop_glue[Internal] local_drop_glue-mod1[Internal]
[01:05:21] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]> @@ local_drop_glue-mod1[Internal]
[01:05:21] MONO_ITEM fn local_drop_glue::mod1[0]::user[0] @@ local_drop_glue-mod1[External]
[01:05:21] MONO_ITEM fn local_drop_glue::user[0] @@ local_drop_glue[External]
[01:05:21] MONO_ITEM fn local_drop_glue::{{impl}}[0]::drop[0] @@ local_drop_glue[External]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] error: aborting due to previous error
[01:05:21] 
[01:05:21] 
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] 
[01:05:21] thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[01:05:21] 
[01:05:21] ---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/inlining-from-extern-crate.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/inlining-from-extern-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/inlining-from-extern-crate" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/inlining-from-extern-crate/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn cgu_explicit_inlining::always_inlined[0] @@ inlining_from_extern_crate[Internal] inlining_from_extern_crate-mod2[Internal]
[01:05:21] MONO_ITEM fn cgu_explicit_inlining::inlined[0] @@ inlining_from_extern_crate[Internal] inlining_from_extern_crate-mod1[Internal]
[01:05:21] MONO_ITEM fn inlining_from_extern_crate::mod1[0]::user[0] @@ inlining_from_extern_crate-mod1[External]
[01:05:21] MONO_ITEM fn inlining_from_extern_crate::mod2[0]::user[0] @@ inlining_from_extern_crate-mod2[External]
[01:05:21] MONO_ITEM fn inlining_from_extern_crate::user[0] @@ inlining_from_extern_crate[External]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread 'thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:21] <unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] error: aborting due to previous error
[01:05:21] 
[01:05:21] 
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] 
[01:05:21] thread '[codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[01:05:21] 
[01:05:21] ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/extern-generic.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zincremental=tmp/partitioning-tests/extern-generic" "-Zshare-generics=y" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/extern-generic/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn cgu_generic_function::bar[0]<&str> @@ cgu_generic_function.volatile[External]
[01:05:21] MONO_ITEM fn cgu_generic_function::foo[0]<&str> @@ cgu_generic_function.volatile[External]
[01:05:21] MONO_ITEM fn extern_generic::mod1[0]::mod1[0]::user[0] @@ extern_generic-mod1-mod1[Internal]
[01:05:21] MONO_ITEM fn extern_generic::mod1[0]::user[0] @@ extern_generic-mod1[Internal]
[01:05:21] MONO_ITEM fn extern_generic::mod2[0]::user[0] @@ extern_generic-mod2[Internal]
[01:05:21] MONO_ITEM fn extern_generic::mod3[0]::non_user[0] @@ extern_generic-mod3[Internal]
[01:05:21] MONO_ITEM fn extern_generic::user[0] @@ extern_generic[Internal]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] error: aborting due to previous error
[01:05:21] 
[01:05:21] 
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] 
[01:05:21] thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[01:05:21] 
[01:05:21] ---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-generic.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-generic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zincremental=tmp/partitioning-tests/local-generic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-generic/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn local_generic::generic[0]<&str> @@ local_generic.volatile[External]
[01:05:21] MONO_ITEM fn local_generic::generic[0]<char> @@ local_generic.volatile[External]
[01:05:21] MONO_ITEM fn local_generic::generic[0]<u32> @@ local_generic.volatile[External]
[01:05:21] MONO_ITEM fn local_generic::generic[0]<u64> @@ local_generic.volatile[External]
[01:05:21] MONO_ITEM fn local_generic::mod1[0]::mod1[0]::user[0] @@ local_generic-mod1-mod1[Internal]
[01:05:21] MONO_ITEM fn local_generic::mod1[0]::user[0] @@ local_generic-mod1[Internal]
[01:05:21] MONO_ITEM fn local_generic::mod2[0]::user[0] @@ local_generic-mod2[Internal]
[01:05:21] MONO_ITEM fn local_generic::user[0] @@ local_generic[Internal]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] 
[01:05:21] error: aborting due to previous error
[01:05:21] 
---
[01:05:21] ---- [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-inlining-but-not-all.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining-but-not-all/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-inlining-but-not-all" "-Zinline-in-all-cgus=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining-but-not-all/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn local_inlining_but_not_all::inline[0]::inlined_function[0] @@ local_inlining_but_not_all-inline[External]
[01:05:21] MONO_ITEM fn local_inlining_but_not_all::non_user[0]::baz[0] @@ local_inlining_but_not_all-non_user[External]
[01:05:21] MONO_ITEM fn local_inlining_but_not_all::user1[0]::foo[0] @@ local_inlining_but_not_all-user1[External]
[01:05:21] MONO_ITEM fn local_inlining_but_not_all::user2[0]::bar[0] @@ local_inlining_but_not_all-user2[External]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] 
[01:05:21] error: aborting due to previous error
[01:05:21] 
[01:05:21] 
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] ------------------------------------------
[01:05:21] 
[01:05:21] thread '[codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[01:05:21] 
[01:05:21] 
[01:05:21] ---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-inlining.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-inlining" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn local_inlining::inline[0]::inlined_function[0] @@ local_inlining-user1[Internal] local_inlining-user2[Internal]
[01:05:21] MONO_ITEM fn local_inlining::non_user[0]::baz[0] @@ local_inlining-non_user[External]
[01:05:21] MONO_ITEM fn local_inlining::user1[0]::foo[0] @@ local_inlining-user1[External]
[01:05:21] MONO_ITEM fn local_inlining::user2[0]::bar[0] @@ local_inlining-user2[External]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] 
[01:05:21] error: aborting due to previous error
[01:05:21] 
---
[01:05:21] ---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/local-transitive-inlining.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-transitive-inlining/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/local-transitive-inlining" "-Zinline-in-all-cgus" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-transitive-inlining/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn local_transitive_inlining::direct_user[0]::foo[0] @@ local_transitive_inlining-indirect_user[Internal]
[01:05:21] MONO_ITEM fn local_transitive_inlining::indirect_user[0]::bar[0] @@ local_transitive_inlining-indirect_user[External]
[01:05:21] MONO_ITEM fn local_transitive_inlining::inline[0]::inlined_function[0] @@ local_transitive_inlining-indirect_user[Internal]
[01:05:21] MONO_ITEM fn local_transitive_inlining::non_user[0]::baz[0] @@ local_transitive_inlining-non_user[External]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] 
[01:05:21] error: aborting due to previous error
[01:05:21] 
---
[01:05:21] ---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/regular-modules.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/regular-modules/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zincremental=tmp/partitioning-tests/regular-modules" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/regular-modules/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn regular_modules::bar[0] @@ regular_modules[Internal]
[01:05:21] MONO_ITEM fn regular_modules::foo[0] @@ regular_modules[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod1[0]::bar[0] @@ regular_modules-mod1[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod1[0]::foo[0] @@ regular_modules-mod1[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod1[0]::mod1[0]::bar[0] @@ regular_modules-mod1-mod1[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod1[0]::mod1[0]::foo[0] @@ regular_modules-mod1-mod1[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod1[0]::mod2[0]::bar[0] @@ regular_modules-mod1-mod2[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod1[0]::mod2[0]::foo[0] @@ regular_modules-mod1-mod2[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod2[0]::bar[0] @@ regular_modules-mod2[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod2[0]::foo[0] @@ regular_modules-mod2[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod2[0]::mod1[0]::bar[0] @@ regular_modules-mod2-mod1[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod2[0]::mod1[0]::foo[0] @@ regular_modules-mod2-mod1[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod2[0]::mod2[0]::bar[0] @@ regular_modules-mod2-mod2[Internal]
[01:05:21] MONO_ITEM fn regular_modules::mod2[0]::mod2[0]::foo[0] @@ regular_modules-mod2-mod2[Internal]
[01:05:21] MONO_ITEM static regular_modules::BAZ[0] @@ regular_modules[Internal]
[01:05:21] MONO_ITEM static regular_modules::mod1[0]::BAZ[0] @@ regular_modules-mod1[Internal]
[01:05:21] MONO_ITEM static regular_modules::mod1[0]::mod1[0]::BAZ[0] @@ regular_modules-mod1-mod1[Internal]
[01:05:21] MONO_ITEM static regular_modules::mod1[0]::mod2[0]::BAZ[0] @@ regular_modules-mod1-mod2[Internal]
[01:05:21] MONO_ITEM static regular_modules::mod2[0]::BAZ[0] @@ regular_modules-mod2[Internal]
[01:05:21] MONO_ITEM static regular_modules::mod2[0]::mod1[0]::BAZ[0] @@ regular_modules-mod2-mod1[Internal]
[01:05:21] MONO_ITEM static regular_modules::mod2[0]::mod2[0]::BAZ[0] @@ regular_modules-mod2-mod2[Internal]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] 
[01:05:21] 
[01:05:21] thread 'thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] <unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] 
[01:05:21] 
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] ------------------------------------------
[01:05:21] 
[01:05:21] thread '[codegen-units] codegen-units/partitioning/regular-modules.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[01:05:21] 
[01:05:21] 
[01:05:21] ---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/statics.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/statics/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=lazy" "-Zincremental=tmp/partitioning-tests/statics" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/statics/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn statics::function[0] @@ statics[External]
[01:05:21] MONO_ITEM fn statics::mod1[0]::function[0] @@ statics-mod1[External]
[01:05:21] MONO_ITEM static statics::BAR[0] @@ statics[Internal]
[01:05:21] MONO_ITEM static statics::FOO[0] @@ statics[Internal]
[01:05:21] MONO_ITEM static statics::function[0]::BAR[0] @@ statics[Internal]
[01:05:21] MONO_ITEM static statics::function[0]::FOO[0] @@ statics[Internal]
[01:05:21] MONO_ITEM static statics::mod1[0]::BAR[0] @@ statics-mod1[Internal]
[01:05:21] MONO_ITEM static statics::mod1[0]::FOO[0] @@ statics-mod1[Internal]
[01:05:21] MONO_ITEM static statics::mod1[0]::function[0]::BAR[0] @@ statics-mod1[Internal]
[01:05:21] MONO_ITEM static statics::mod1[0]::function[0]::FOO[0] @@ statics-mod1[Internal]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] warning: static item is never used: `FOO`
[01:05:21]    |
[01:05:21]    |
[01:05:21] 19 | static FOO: u32 = 0;
[01:05:21]    |
[01:05:21]    = note: #[warn(dead_code)] on by default
[01:05:21] 
[01:05:21] 
[01:05:21] warning: static item is never used: `BAR`
[01:05:21]    |
[01:05:21]    |
[01:05:21] 22 | static BAR: u32 = 0;
[01:05:21] 
[01:05:21] 
[01:05:21] warning: static item is never used: `FOO`
[01:05:21]    |
[01:05:21]    |
[01:05:21] 27 |     static FOO: u32 = 0;
[01:05:21] 
[01:05:21] 
[01:05:21] warning: static item is never used: `BAR`
[01:05:21]    |
[01:05:21]    |
[01:05:21] 30 |     static BAR: u32 = 0;
[01:05:21] 
[01:05:21] 
[01:05:21] warning: static item is never used: `FOO`
[01:05:21]    |
[01:05:21]    |
[01:05:21] 35 |     static FOO: u32 = 0;
[01:05:21] 
[01:05:21] 
[01:05:21] warning: static item is never used: `BAR`
[01:05:21]    |
[01:05:21]    |
[01:05:21] 38 |     static BAR: u32 = 0;
[01:05:21] 
[01:05:21] 
[01:05:21] warning: static item is never used: `FOO`
[01:05:21]    |
[01:05:21]    |
[01:05:21] 43 |         static FOO: u32 = 0;
[01:05:21] 
[01:05:21] 
[01:05:21] warning: static item is never used: `BAR`
[01:05:21]    |
[01:05:21]    |
[01:05:21] 46 |         static BAR: u32 = 0;
[01:05:21] 
[01:05:21] 
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] 
[01:05:21] error: aborting due to previous error
[01:05:21] 
---
[01:05:21] ---- [codegen-units] codegen-units/partitioning/shared-generics.rs stdout ----
[01:05:21] 
[01:05:21] error: compilation failed!
[01:05:21] status: exit code: 101
[01:05:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen-units/partitioning/shared-generics.rs" "--target=x86_64-unknown-linux-gnu" "-Z" "human_readable_cgu_names" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/shared-generics/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zprint-mono-items=eager" "-Zshare-generics=yes" "-Zincremental=tmp/partitioning-tests/shared-generics-exe" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units/partitioning/shared-generics/auxiliary"
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] MONO_ITEM fn shared_generics::foo[0] @@ shared_generics[External]
[01:05:21] MONO_ITEM fn shared_generics_aux::generic_fn[0]<u16> @@ shared_generics_aux.volatile[External]
[01:05:21] ------------------------------------------
[01:05:21] stderr:
[01:05:21] ------------------------------------------
[01:05:21] ------------------------------------------
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:21] thread '<unnamed>' panicked at 'assertion failed: addpass("name-anon-globals")', librustc_codegen_llvm/back/write.rs:557:17
[01:05:21] error: aborting due to worker thread failure
[01:05:21] error: aborting due to previous error
[01:05:21] 
[01:05:21] 
[01:05:21] ------------------------------------------
---
[01:05:21] test result: FAILED. 25 passed; 11 failed; 3 ignored; 0 measured; 0 filtered out
[01:05:21] 
[01:05:21] 
[01:05:21] 
[01:05:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:21] 
[01:05:21] 
[01:05:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:21] Build completed unsuccessfully in 0:16:31
[01:05:21] Build completed unsuccessfully in 0:16:31
[01:05:21] Makefile:58: recipe for target 'check' failed
[01:05:21] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fe0f7c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
