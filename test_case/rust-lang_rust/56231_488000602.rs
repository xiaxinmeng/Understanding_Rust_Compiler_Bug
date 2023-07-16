plain
travis_time:end:2b8a417c:start=1556633528845932045,finish=1556633529715017902,duration=869085857
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:32] 
[01:21:32] running 139 tests
[01:21:34] i..iii..F..iiiF..iiii....i.......F.F...F......i..iF.........F.....Fi....FiF...F.....ii.i..i.Fi.ii... 100/139
[01:21:36] .F.........i.........ii.i.....iii...F..
[01:21:36] 
[01:21:36] ---- [codegen] codegen/adjustments.rs stdout ----
[01:21:36] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:21:36] 
[01:21:36] 
[01:21:36] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll" "/checkout/src/test/codegen/adjustments.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/adjustments.rs:16:11: error: expected string not found in input
[01:21:36] // CHECK: %0 = insertvalue { [0 x i8]*, [[USIZE]] } undef, [0 x i8]* %x.0, 0
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:13:50: note: scanning from here
[01:21:36] define { [0 x i8]*, i64 } @no_op_slice_adjustment([0 x i8]* noalias nonnull readonly align 1 %arg0.0, i64 %arg0.1) unnamed_addr #0 {
[01:21:36]                                                  ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:13:50: note: with variable "USIZE" equal to "i64"
[01:21:36] define { [0 x i8]*, i64 } @no_op_slice_adjustment([0 x i8]* noalias nonnull readonly align 1 %arg0.0, i64 %arg0.1) unnamed_addr #0 {
[01:21:36]                                                  ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:15:2: note: possible intended match here
[01:21:36]  %0 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %arg0.0, 0
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/c-variadic.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/c-variadic/c-variadic.ll" "/checkout/src/test/codegen/c-variadic.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/c-variadic.rs:28:12: error: expected string not found in input
[01:21:36]  // CHECK: invoke void ({{.*}}*, ...) @foreign_c_variadic_1({{.*}} %ap)
[01:21:36]            ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/c-variadic/c-variadic.ll:853:2: note: scanning from here
[01:21:36]  to label %bb5 unwind label %cleanup
[01:21:36]  ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/c-variadic/c-variadic.ll:877:2: note: possible intended match here
[01:21:36]  invoke void (i64*, ...) @foreign_c_variadic_1(i64* align 8 dereferenceable(24) %arg0)
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/fn-impl-trait-self.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fn-impl-trait-self/fn-impl-trait-self.ll" "/checkout/src/test/codegen/fn-impl-trait-self.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/fn-impl-trait-self.rs:4:11: error: expected string not found in input
[01:21:36] // CHECK: {{.*}}DIDerivedType(tag: DW_TAG_pointer_type, name: "fn() -> <recursive_type>",{{.*}}
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fn-impl-trait-self/fn-impl-trait-self.ll:77:17: note: scanning from here
[01:21:36] define i32 @main(i32, i8**) unnamed_addr #4 {
[01:21:36]                 ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fn-impl-trait-self/fn-impl-trait-self.ll:125:2: note: possible intended match here
[01:21:36] !8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn()", baseType: !9, size: 64, align: 64)
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/function-arguments.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll" "/checkout/src/test/codegen/function-arguments.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/function-arguments.rs:15:11: error: expected string not found in input
[01:21:36] // CHECK: zeroext i1 @boolean(i1 zeroext %x)
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:1:1: note: scanning from here
[01:21:36] ; ModuleID = 'function_arguments.3a1fbbbh-cgu.0'
[01:21:36] ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:88:109: note: possible intended match here
[01:21:36]  call void @_ZN5alloc5alloc8box_free17hda377f3dbfb4d68bE(i8* nonnull %16, i64* noalias readonly align 8 dereferenceable(24) %18)
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/generic-debug.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generic-debug/generic-debug.ll" "/checkout/src/test/codegen/generic-debug.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/generic-debug.rs:6:11: error: expected string not found in input
[01:21:36] // CHECK: {{.*}}DICompositeType{{.*}}tag: DW_TAG_structure_type,{{.*}}name: "Generic<i32>",{{.*}}
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generic-debug/generic-debug.ll:209:17: note: scanning from here
[01:21:36] define i32 @main(i32, i8**) unnamed_addr #4 {
[01:21:36]                 ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generic-debug/generic-debug.ll:238:1: note: possible intended match here
[01:21:36] !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "vtable", file: !2, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !5, identifier: "vtable")
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/issue-32031.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-32031/issue-32031.ll" "/checkout/src/test/codegen/issue-32031.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/issue-32031.rs:8:11: error: expected string not found in input
[01:21:36] // CHECK: define float @add_newtype_f32(float %a, float %b)
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-32031/issue-32031.ll:1:1: note: scanning from here
[01:21:36] ; ModuleID = 'issue_32031.3a1fbbbh-cgu.0'
[01:21:36] ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-32031/issue-32031.ll:7:1: note: possible intended match here
[01:21:36] define float @add_newtype_f32(float %arg0, float %arg1) unnamed_addr #0 {
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/lifetime_start_end.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end/lifetime_start_end.ll" "/checkout/src/test/codegen/lifetime_start_end.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/lifetime_start_end.rs:11:11: error: expected string not found in input
[01:21:36] // CHECK: [[S_a:%[0-9]+]] = bitcast i32* %a to i8*
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end/lifetime_start_end.ll:9:18: note: scanning from here
[01:21:36] define void @test() unnamed_addr #0 {
[01:21:36]                  ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end/lifetime_start_end.ll:15:2: note: possible intended match here
[01:21:36]  %0 = bitcast i32* %_1 to i8*
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/loads.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll" "/checkout/src/test/codegen/loads.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/loads.rs:15:11: error: expected string not found in input
[01:21:36] // CHECK: load {{(i32\*, )?}}i32** %x{{.*}}, !nonnull
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:193:47: note: scanning from here
[01:21:36] define align 4 dereferenceable(4) i32* @borrow(i32* noalias readonly align 4 dereferenceable(4)) unnamed_addr #1 {
[01:21:36]                                               ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:197:7: note: possible intended match here
[01:21:36]  %1 = load i32*, i32** %arg0, align 8, !nonnull !2
[01:21:36] /checkout/src/test/codegen/loads.rs:23:11: error: expected string not found in input
[01:21:36] /checkout/src/test/codegen/loads.rs:23:11: error: expected string not found in input
[01:21:36] // CHECK: load {{(i32\*, )?}}i32** %x{{.*}}, !nonnull
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:202:17: note: scanning from here
[01:21:36] define i32 @_box(i32* noalias align 4 dereferenceable(4)) unnamed_addr #1 {
[01:21:36]                 ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:206:7: note: possible intended match here
[01:21:36]  %1 = load i32*, i32** %arg0, align 8, !nonnull !2
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/move-val-init.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/move-val-init/move-val-init.ll" "/checkout/src/test/codegen/move-val-init.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/move-val-init.rs:17:12: error: expected string not found in input
[01:21:36]  // CHECK: call void %make_big(%Big*{{[^%]*}} %target)
[01:21:36]            ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/move-val-init/move-val-init.ll:9:22: note: scanning from here
[01:21:36] define void @test_mvi(%Big* %arg0, void (%Big*)* nonnull %arg1) unnamed_addr #0 {
[01:21:36]                      ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/move-val-init/move-val-init.ll:11:2: note: possible intended match here
[01:21:36]  call void %arg1(%Big* noalias nocapture sret dereferenceable(65536) %arg0)
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/naked-functions.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll" "/checkout/src/test/codegen/naked-functions.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/naked-functions.rs:23:17: error: expected string not found in input
[01:21:36]  // CHECK-NEXT: %a = alloca i{{[0-9]+}}
[01:21:36]                 ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:15:2: note: scanning from here
[01:21:36]  %arg0 = alloca i64, align 8
[01:21:36]  ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:15:5: note: possible intended match here
[01:21:36]  %arg0 = alloca i64, align 8
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/nontemporal.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/nontemporal/nontemporal.ll" "/checkout/src/test/codegen/nontemporal.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/nontemporal.rs:9:12: error: expected string not found in input
[01:21:36]  // CHECK: store i32 %b, i32* %a, align 4, !nontemporal
[01:21:36]            ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/nontemporal/nontemporal.ll:7:15: note: scanning from here
[01:21:36] define void @a(i32* nocapture align 4 dereferenceable(4) %arg0, i32 %arg1) unnamed_addr #0 {
[01:21:36]               ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/nontemporal/nontemporal.ll:9:2: note: possible intended match here
[01:21:36]  store i32 %arg1, i32* %arg0, align 4, !nontemporal !1
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/refs.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/refs/refs.ll" "/checkout/src/test/codegen/refs.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/refs.rs:16:11: error: expected string not found in input
[01:21:36] // CHECK: [[X0:%[0-9]+]] = getelementptr {{.*}} { [0 x i8]*, [[USIZE]] }* %x, i32 0, i32 0
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/refs/refs.ll:13:21: note: scanning from here
[01:21:36] define void @ref_dst([0 x i8]* noalias nonnull readonly align 1 %arg0.0, i64 %arg0.1) unnamed_addr #0 {
[01:21:36]                     ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/refs/refs.ll:13:21: note: with variable "USIZE" equal to "i64"
[01:21:36] define void @ref_dst([0 x i8]* noalias nonnull readonly align 1 %arg0.0, i64 %arg0.1) unnamed_addr #0 {
[01:21:36]                     ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/refs/refs.ll:18:30: note: possible intended match here
[01:21:36]  %1 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %_2, i32 0, i32 0
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/scalar-pair-bool.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/scalar-pair-bool/scalar-pair-bool.ll" "/checkout/src/test/codegen/scalar-pair-bool.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/scalar-pair-bool.rs:5:11: error: expected string not found in input
[01:21:36] // CHECK: define { i8, i8 } @pair_bool_bool(i1 zeroext %pair.0, i1 zeroext %pair.1)
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/scalar-pair-bool/scalar-pair-bool.ll:1:1: note: scanning from here
[01:21:36] ; ModuleID = 'scalar_pair_bool.3a1fbbbh-cgu.0'
[01:21:36] ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/scalar-pair-bool/scalar-pair-bool.ll:15:1: note: possible intended match here
[01:21:36] define { i8, i8 } @pair_bool_bool(i1 zeroext %arg0.0, i1 zeroext %arg0.1) unnamed_addr #0 {
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] ---- [codegen] codegen/union-abi.rs stdout ----
[01:21:36] 
[01:21:36] error: verification with 'FileCheck' failed
[01:21:36] status: exit code: 1
[01:21:36] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll" "/checkout/src/test/codegen/union-abi.rs"
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] stderr:
[01:21:36] stderr:
[01:21:36] ------------------------------------------
[01:21:36] /checkout/src/test/codegen/union-abi.rs:72:11: error: expected string not found in input
[01:21:36] // CHECK: define zeroext i1 @test_UnionBool(i8 %b)
[01:21:36]           ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll:95:88: note: scanning from here
[01:21:36] define void @test_CUnionU128(%CUnionU128* noalias nocapture dereferenceable(16) %arg0) unnamed_addr #0 {
[01:21:36]                                                                                        ^
[01:21:36] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll:104:1: note: possible intended match here
[01:21:36] define zeroext i1 @test_UnionBool(i8 %arg0) unnamed_addr #0 {
[01:21:36] 
[01:21:36] ------------------------------------------
[01:21:36] 
[01:21:36] 
---
[01:21:36] test result: FAILED. 95 passed; 14 failed; 30 ignored; 0 measured; 0 filtered out
[01:21:36] 
[01:21:36] 
[01:21:36] 
[01:21:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:36] 
[01:21:36] 
[01:21:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:36] Build completed unsuccessfully in 0:11:51
[01:21:36] Build completed unsuccessfully in 0:11:51
[01:21:36] Makefile:48: recipe for target 'check' failed
[01:21:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05ed32b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 15:33:57 UTC 2019
---
travis_time:end:0184eaaa:start=1556638439167536909,finish=1556638439172128581,duration=4591672
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f2f4958
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c426942
travis_time:start:0c426942
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b499372
$ dmesg | grep -i kill
