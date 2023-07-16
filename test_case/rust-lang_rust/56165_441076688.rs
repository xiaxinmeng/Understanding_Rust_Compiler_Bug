plain
travis_time:end:152e3c70:start=1542899738760911685,finish=1542899792935674515,duration=54174762830
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:14] .................................................................................................... 100/5048
[00:49:16] .................................................................................................... 200/5048
[00:49:19] .............................ii............................................ii...................ii.. 300/5048
[00:49:22] ..............................................................................................iii... 400/5048
[00:49:24] .....iiiiiiii.iii............................iii...........................................i........ 500/5048
[00:49:31] .................................................................................................... 700/5048
[00:49:37] ...................................................................................i...........i.... 800/5048
[00:49:41] .................................................................................................... 900/5048
[00:49:44] ..iiiii..................ii.iiii.................................................................... 1000/5048
---
[00:50:19] .................................................................................................... 2200/5048
[00:50:23] .................................................................................................... 2300/5048
[00:50:26] .................................................................................................... 2400/5048
[00:50:30] .................................................................................................... 2500/5048
[00:50:33] ...........................................................................................iiiiiiiii 2600/5048
[00:50:40] .........................................................ii......................................... 2800/5048
[00:50:43] .................................................................................................... 2900/5048
[00:50:47] .................................................................................................... 3000/5048
[00:50:50] .....................................................i.............................................. 3100/5048
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:18] 
[01:04:18] running 117 tests
[01:04:21] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:04:22] i.i.....iiii.....
[01:04:22] 
[01:04:22]  finished in 3.443
[01:04:22] travis_fold:end:test_codegen


[01:04:22] travis_time:end:test_codegen:start=1542903660893407241,finish=1542903664336715420,duration=3443308179

[01:04:22] travis_fold:start:test_codegen-units
travis_time:start:test_codegen-units
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:22] 
[01:04:22] running 39 tests
[01:04:23] iF....F.i.F.F........FF..FFF...i.....F.
[01:04:23] 
[01:04:23] ---- [codegen-units] codegen-units/item-collection/drop_in_place_intrinsic.rs stdout ----
[01:04:23] 
[01:04:23] 
[01:04:23] These items should have been contained but were not:
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]; 2]> @@ drop_in_place_intrinsic-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<drop_in_place_intrinsic::StructWithDtor[0]> @@ drop_in_place_intrinsic-cgu.0[Internal]
[01:04:23] 
[01:04:23] 
[01:04:23] These items were contained but should not have been:
[01:04:23] 
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]; 2]> @@ drop_in_place_intrinsic.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<[drop_in_place_intrinsic::StructWithDtor[0]]> @@ drop_in_place_intrinsic.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<drop_in_place_intrinsic::StructWithDtor[0]> @@ drop_in_place_intrinsic.7rcbfp3g-cgu.0[Internal]
[01:04:23] 
[01:04:23] thread '[codegen-units] codegen-units/item-collection/drop_in_place_intrinsic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2364:13
[01:04:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:23] 
[01:04:23] 
[01:04:23] ---- [codegen-units] codegen-units/item-collection/generic-drop-glue.rs stdout ----
[01:04:23] 
[01:04:23] These items should have been contained but were not:
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<f64, f32>> @@ generic_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<i32, i64>> @@ generic_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::NonGenericWithDrop[0]> @@ generic_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructNoDrop[0]<generic_drop_glue::NonGenericWithDrop[0], f64>> @@ generic_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<&str, generic_drop_glue::NonGenericNoDrop[0]>> @@ generic_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<generic_drop_glue::StructWithDrop[0]<i8, char>> @@ generic_drop_glue-cgu.0[Internal]
[01:04:23] 
[01:04:23] 
[01:04:23] These items were contained but should not have been:
[01:04:23] 
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<generic_drop_glue::EnumWithDrop[0]<f64, f32>> @@ generic_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<generic_drop_glue::Evtable::Struct[0]<u32>> @@ instantiation_through_vtable.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<instantiation_through_vtable::Struct[0]<u64>> @@ instantiation_through_vtable.7rcbfp3g-cgu.0[Internal]
[01:04:23] 
[01:04:23] thread '[codegen-units] codegen-units/item-collection/instantiation-through-vtable.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2364:13
[01:04:23] 
[01:04:23] ---- [codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs stdout ----
[01:04:23] ---- [codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs stdout ----
[01:04:23] 
[01:04:23] These items should have been contained but were not:
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<non_generic_drop_glue::EnumWithDrop[0]> @@ non_generic_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<non_generic_drop_glue::StructWithDrop[0]> @@ non_generic_drop_glue-cgu.0[Internal]
[01:04:23] 
[01:04:23] 
[01:04:23] These items were contained but should not have been:
[01:04:23] 
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<non_generic_drop_glue::EnumWithDrop[0]> @@ non_generic_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<non_generic_drop_glue::StructWithDrop[0]> @@ non_generic_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] 
[01:04:23] thread '[codegen-units] codegen-units/item-collection/non-generic-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2364:13
[01:04:23] 
[01:04:23] ---- [codegen-units] codegen-units/item-collection/transitive-drop-glue.rs stdout ----
[01:04:23] ---- [codegen-units] codegen-units/item-collection/transitive-drop-glue.rs stdout ----
[01:04:23] 
[01:04:23] These items should have been contained but were not:
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<i16>> @@ transitive_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<u32>> @@ transitive_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Intermediate[0]> @@ transitive_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::LeafGen[0]<i16>> @@ transitive_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::LeafGen[0]<u32>> @@ transitive_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Leaf[0]> @@ transitive_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::RootGen[0]<i16>> @@ transitive_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::RootGen[0]<u32>> @@ transitive_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<transitive_drop_glue::Root[0]> @@ transitive_drop_glue-cgu.0[Internal]
[01:04:23] 
[01:04:23] 
[01:04:23] These items were contained but should not have been:
[01:04:23] 
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<i16>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::IntermediateGen[0]<u32>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::Intermediate[0]> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::LeafGen[0]<i16>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::LeafGen[0]<u32>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::Leaf[0]> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::RootGen[0]<i16>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::RootGen[0]<u32>> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<transitive_drop_glue::Root[0]> @@ transitive_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] 
[01:04:23] thread '[codegen-units] codegen-units/item-collection/transitive-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2364:13
[01:04:23] 
[01:04:23] ---- [codegen-units] codegen-units/item-collection/tuple-drop-glue.rs stdout ----
[01:04:23] ---- [codegen-units] codegen-units/item-collection/tuple-drop-glue.rs stdout ----
[01:04:23] 
[01:04:23] These items should have been contained but were not:
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(i16, (tuple_drop_glue::Dropped[0], bool))> @@ tuple_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(tuple_drop_glue::Dropped[0], bool)> @@ tuple_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<(u32, tuple_drop_glue::Dropped[0])> @@ tuple_drop_glue-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<tuple_drop_glue::Dropped[0]> @@ tuple_drop_glue-cgu.0[Internal]
[01:04:23] 
[01:04:23] 
[01:04:23] These items were contained but should not have been:
[01:04:23] 
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<(i16, (tuple_drop_glue::Dropped[0], bool))> @@ tuple_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<(tuple_drop_glue::Dropped[0], bool)> @@ tuple_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<(u32, tuple_drop_glue::Dropped[0])> @@ tuple_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<tuple_drop_glue::Dropped[0]> @@ tuple_drop_glue.7rcbfp3g-cgu.0[Internal]
[01:04:23] 
[01:04:23] thread '[codegen-units] codegen-units/item-collection/tuple-drop-glue.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2364:13
[01:04:23] 
[01:04:23] ---- [codegen-units] codegen-units/item-collection/unsizing.rs stdout ----
[01:04:23] ---- [codegen-units] codegen-units/item-collection/unsizing.rs stdout ----
[01:04:23] 
[01:04:23] These items should have been contained but were not:
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<bool> @@ unsizing-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<char> @@ unsizing-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<f64> @@ unsizing-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<u32> @@ unsizing-cgu.0[Internal]
[01:04:23] 
[01:04:23] 
[01:04:23] These items were contained but should not have been:
[01:04:23] 
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<bool> @@ unsizing.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<char> @@ unsizing.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<f64> @@ unsizing.7rcbfp3g-cgu.0[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<u32> @@ unsizing.7rcbfp3g-cgu.0[Internal]
[01:04:23] 
[01:04:23] thread '[codegen-units] codegen-units/item-collection/unsizing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2364:13
[01:04:23] 
[01:04:23] ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
[01:04:23] ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
[01:04:23] 
[01:04:23] These items should have been contained but were not:
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<cgu_extern_drop_glue::Struct[0]> @@ extern_drop_glue[Internal] extern_drop_glue-mod1[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::LocalStruct[0]> @@ extern_drop_glue[Internal]
[01:04:23] MONO_ITEM fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]> @@ extern_drop_glue-mod1[Internal]
[01:04:23] 
[01:04:23] 
[01:04:23] These items were contained but should not have been:
[01:04:23] 
[01:04:23] 
[01:04:23] MONO_ITEM fn core::ptr[0]::real_drop_in_place[0]<cgu_extern_drop_glue::Struct[0]> @@ extern_drop_glue.3a1fbbbhn-units] codegen-units/item-collection/tuple-drop-glue.rs
[01:04:23]     [codegen-units] codegen-units/partitioning/extern-drop-glue.rs
[01:04:23]     [codegen-units] codegen-units/partitioning/local-drop-glue.rs
[01:04:23]     [codegen-units] codegen-units/partitioning/vtable-through-const.rs
[01:04:23] 
[01:04:23] 
[01:04:23] test result: FAILED. 26 passed; 10 failed; 3 ignored; 0 measured; 0 filtered out
[01:04:23] 
[01:04:23] 
[01:04:23] 
[01:04:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:23] 
[01:04:23] 
[01:04:23] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:04:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:23] Build completed unsuccessfully in 0:18:58
[01:04:23] make: *** [check] Error 1
[01:04:23] Makefile:58: recipe for target 'check' failed
