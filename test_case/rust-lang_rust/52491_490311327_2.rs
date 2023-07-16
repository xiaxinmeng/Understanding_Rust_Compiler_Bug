
---- [codegen] codegen/remap_path_prefix/main.rs stdout ----


executing "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/stage2/bin/rustc" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/remap_path_prefix/auxiliary/remap_path_prefix_aux.rs" "--target=s390x-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/native/rust-test-helpers" "-g" "--remap-path-prefix=/<<BUILDDIR>>/rustc-1.32.0+dfsg1=/the/aux-cwd" "--remap-path-prefix=/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/remap_path_prefix/auxiliary=/the/aux-src" "--crate-type" "dylib" "-L" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main/auxiliary"
------stdout------------------------------

------stderr------------------------------

------------------------------------------
executing "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/stage2/bin/rustc" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/remap_path_prefix/main.rs" "--target=s390x-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/native/rust-test-helpers" "-g" "-C" "no-prepopulate-passes" "--remap-path-prefix=/<<BUILDDIR>>/rustc-1.32.0+dfsg1=/the/cwd" "--remap-path-prefix=/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen=/the/src" "-L" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main/auxiliary" "--emit=llvm-ir"
------stdout------------------------------

------stderr------------------------------

------------------------------------------
executing "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/remap_path_prefix/main.rs"
------stdout------------------------------

------stderr------------------------------
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/remap_path_prefix/main.rs:25:11: error: CHECK: expected string not found in input
// CHECK: @0 = private unnamed_addr constant <{ [34 x i8] }> <{ [34 x i8] c"/the/src/remap_path_prefix/main.rs" }>, align 1
          ^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:1:1: note: scanning from here
; ModuleID = 'main.7rcbfp3g-cgu.0'
^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:10:1: note: possible intended match here
@0 = private unnamed_addr constant <{ [34 x i8] }> <{ [34 x i8] c"/the/src/remap_path_prefix/main.rs" }>, align 2
^

------------------------------------------

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/remap_path_prefix/main.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/remap_path_prefix/main.rs:25:11: error: CHECK: expected string not found in input
// CHECK: @0 = private unnamed_addr constant <{ [34 x i8] }> <{ [34 x i8] c"/the/src/remap_path_prefix/main.rs" }>, align 1
          ^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:1:1: note: scanning from here
; ModuleID = 'main.7rcbfp3g-cgu.0'
^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:10:1: note: possible intended match here
@0 = private unnamed_addr constant <{ [34 x i8] }> <{ [34 x i8] c"/the/src/remap_path_prefix/main.rs" }>, align 2
^

------------------------------------------

thread '[codegen] codegen/remap_path_prefix/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [codegen] codegen/repr-transparent.rs stdout ----


executing "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/stage2/bin/rustc" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/repr-transparent.rs" "--target=s390x-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/repr-transparent" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-L" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/repr-transparent/auxiliary" "--emit=llvm-ir"
------stdout------------------------------

------stderr------------------------------
warning: field is never used: `val`
  --> /<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/repr-transparent.rs:76:32
   |
76 | pub struct UnitPhantom<T, U> { val: T, unit: PhantomData<U> }
   |                                ^^^^^^
   |
   = note: #[warn(dead_code)] on by default

warning: type `f32x4` should have a camel case name such as `F32x4`
   --> /<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/repr-transparent.rs:106:1
    |
106 | struct f32x4(f32, f32, f32, f32);
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: #[warn(non_camel_case_types)] on by default


------------------------------------------
executing "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/repr-transparent/repr-transparent.ll" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/repr-transparent.rs"
------stdout------------------------------

------stderr------------------------------
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/repr-transparent.rs:111:11: error: CHECK: expected string not found in input
// CHECK: define <4 x float> @test_Vector(<4 x float> %arg0)
          ^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/repr-transparent/repr-transparent.ll:97:43: note: scanning from here
define double @test_Nested2(double %arg0) unnamed_addr #0 {
                                          ^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/repr-transparent/repr-transparent.ll:106:1: note: possible intended match here
define void @test_Vector(<4 x float>* noalias nocapture sret dereferenceable(16), <4 x float>* noalias nocapture dereferenceable(16) %arg0) unnamed_addr #0 {
^

------------------------------------------

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/repr-transparent/repr-transparent.ll" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/repr-transparent.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/repr-transparent.rs:111:11: error: CHECK: expected string not found in input
// CHECK: define <4 x float> @test_Vector(<4 x float> %arg0)
          ^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/repr-transparent/repr-transparent.ll:97:43: note: scanning from here
define double @test_Nested2(double %arg0) unnamed_addr #0 {
                                          ^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/repr-transparent/repr-transparent.ll:106:1: note: possible intended match here
define void @test_Vector(<4 x float>* noalias nocapture sret dereferenceable(16), <4 x float>* noalias nocapture dereferenceable(16) %arg0) unnamed_addr #0 {
^

------------------------------------------

thread '[codegen] codegen/repr-transparent.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9

---- [codegen] codegen/x86_mmx.rs stdout ----


executing "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/stage2/bin/rustc" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/x86_mmx.rs" "--target=s390x-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/x86_mmx" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/x86_mmx/auxiliary" "--emit=llvm-ir"
------stdout------------------------------

------stderr------------------------------
warning: type `i8x8` should have a camel case name such as `I8x8`
  --> /<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/x86_mmx.rs:28:1
   |
28 | pub struct i8x8(u64);
   | ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(non_camel_case_types)] on by default


------------------------------------------
executing "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/x86_mmx/x86_mmx.ll" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/x86_mmx.rs"
------stdout------------------------------

------stderr------------------------------
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/x86_mmx.rs:32:18: error: CHECK-LABEL: expected string not found in input
 // CHECK-LABEL: define void @a(x86_mmx*{{.*}}, x86_mmx*{{.*}}, x86_mmx*{{.*}})
                 ^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/x86_mmx/x86_mmx.ll:1:1: note: scanning from here
; ModuleID = 'x86_mmx.3a1fbbbh-cgu.0'
^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/x86_mmx/x86_mmx.ll:7:1: note: possible intended match here
define void @a(<1 x i64>* noalias nocapture sret dereferenceable(8), <1 x i64>* nocapture dereferenceable(8) %a, <1 x i64>* noalias nocapture readonly dereferenceable(8) %b) unnamed_addr #0 {
^

------------------------------------------

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/x86_mmx/x86_mmx.ll" "/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/x86_mmx.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/src/test/codegen/x86_mmx.rs:32:18: error: CHECK-LABEL: expected string not found in input
 // CHECK-LABEL: define void @a(x86_mmx*{{.*}}, x86_mmx*{{.*}}, x86_mmx*{{.*}})
                 ^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/x86_mmx/x86_mmx.ll:1:1: note: scanning from here
; ModuleID = 'x86_mmx.3a1fbbbh-cgu.0'
^
/<<BUILDDIR>>/rustc-1.32.0+dfsg1/build/s390x-unknown-linux-gnu/test/codegen/x86_mmx/x86_mmx.ll:7:1: note: possible intended match here
define void @a(<1 x i64>* noalias nocapture sret dereferenceable(8), <1 x i64>* nocapture dereferenceable(8) %a, <1 x i64>* noalias nocapture readonly dereferenceable(8) %b) unnamed_addr #0 {
^

------------------------------------------

thread '[codegen] codegen/x86_mmx.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9


failures:
    [codegen] codegen/remap_path_prefix/main.rs
    [codegen] codegen/repr-transparent.rs
    [codegen] codegen/x86_mmx.rs

test result: FAILED. 93 passed; 3 failed; 25 ignored; 0 measured; 0 filtered out
