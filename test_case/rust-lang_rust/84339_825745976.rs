plain
failures:

---- [codegen] codegen/wasm_casts_trapping.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll" "/checkout/src/test/codegen/wasm_casts_trapping.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/wasm_casts_trapping.rs:10:12: error: CHECK: expected string not found in input
 // CHECK: {{.*}} call {{.*}} @llvm.wasm.trunc.{{.*}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:7:35: note: scanning from here
define dso_local i64 @cast_f64_i64(double %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:9:7: note: possible intended match here
 %0 = tail call i64 @llvm.fptosi.sat.i64.f64(double %a)
      ^
/checkout/src/test/codegen/wasm_casts_trapping.rs:19:12: error: CHECK: expected string not found in input
 // CHECK: {{.*}} call {{.*}} @llvm.wasm.trunc.{{.*}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:14:35: note: scanning from here
define dso_local i32 @cast_f64_i32(double %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:16:7: note: possible intended match here
 %0 = tail call i32 @llvm.fptosi.sat.i32.f64(double %a)
      ^
/checkout/src/test/codegen/wasm_casts_trapping.rs:28:12: error: CHECK: expected string not found in input
 // CHECK: {{.*}} call {{.*}} @llvm.wasm.trunc.{{.*}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:21:35: note: scanning from here
define dso_local i64 @cast_f32_i64(float %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:23:7: note: possible intended match here
 %0 = tail call i64 @llvm.fptosi.sat.i64.f32(float %a)
      ^
/checkout/src/test/codegen/wasm_casts_trapping.rs:37:12: error: CHECK: expected string not found in input
 // CHECK: {{.*}} call {{.*}} @llvm.wasm.trunc.{{.*}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:28:35: note: scanning from here
define dso_local i32 @cast_f32_i32(float %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:30:7: note: possible intended match here
 %0 = tail call i32 @llvm.fptosi.sat.i32.f32(float %a)
      ^
/checkout/src/test/codegen/wasm_casts_trapping.rs:46:12: error: CHECK: expected string not found in input
 // CHECK: {{.*}} call {{.*}} @llvm.wasm.trunc.{{.*}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:35:35: note: scanning from here
define dso_local i64 @cast_f64_u64(double %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:37:7: note: possible intended match here
 %0 = tail call i64 @llvm.fptoui.sat.i64.f64(double %a)
      ^
/checkout/src/test/codegen/wasm_casts_trapping.rs:55:12: error: CHECK: expected string not found in input
 // CHECK: {{.*}} call {{.*}} @llvm.wasm.trunc.{{.*}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:42:35: note: scanning from here
define dso_local i32 @cast_f64_u32(double %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:44:7: note: possible intended match here
 %0 = tail call i32 @llvm.fptoui.sat.i32.f64(double %a)
      ^
/checkout/src/test/codegen/wasm_casts_trapping.rs:64:12: error: CHECK: expected string not found in input
 // CHECK: {{.*}} call {{.*}} @llvm.wasm.trunc.{{.*}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:49:35: note: scanning from here
define dso_local i64 @cast_f32_u64(float %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:51:7: note: possible intended match here
 %0 = tail call i64 @llvm.fptoui.sat.i64.f32(float %a)
      ^
/checkout/src/test/codegen/wasm_casts_trapping.rs:73:12: error: CHECK: expected string not found in input
 // CHECK: {{.*}} call {{.*}} @llvm.wasm.trunc.{{.*}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:56:35: note: scanning from here
define dso_local i32 @cast_f32_u32(float %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:58:7: note: possible intended match here
 %0 = tail call i32 @llvm.fptoui.sat.i32.f32(float %a)
      ^
/checkout/src/test/codegen/wasm_casts_trapping.rs:81:12: error: CHECK: expected string not found in input
 // CHECK: fptoui float {{.*}} to i8
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:63:33: note: scanning from here
define dso_local i8 @cast_f32_u8(float %a) unnamed_addr #0 {
                                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll:65:40: note: possible intended match here
 %0 = tail call i8 @llvm.fptoui.sat.i8.f32(float %a)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_trapping/wasm_casts_trapping.ll
Check file: /checkout/src/test/codegen/wasm_casts_trapping.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'wasm_casts_trapping.3a1fbbbh-cgu.0'
            2: source_filename = "wasm_casts_trapping.3a1fbbbh-cgu.0"
            3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
            4: target triple = "wasm32-unknown-emscripten"
            5: 
            6: ; Function Attrs: nounwind readnone uwtable willreturn
            7: define dso_local i64 @cast_f64_i64(double %a) unnamed_addr #0 {
check:10'0                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            8: start:
check:10'0     ~~~~~~
            9:  %0 = tail call i64 @llvm.fptosi.sat.i64.f64(double %a)
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:10'1           ?                                                 possible intended match
           10:  ret i64 %0
check:10'0     ~~~~~~~~~~~
           11: }
check:10'0     ~
           12: 
check:10'0     ~
           13: ; Function Attrs: nounwind readnone uwtable willreturn
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14: define dso_local i32 @cast_f64_i32(double %a) unnamed_addr #0 {
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:19'0                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           15: start:
check:19'0     ~~~~~~
           16:  %0 = tail call i32 @llvm.fptosi.sat.i32.f64(double %a)
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:19'1           ?                                                 possible intended match
           17:  ret i32 %0
check:19'0     ~~~~~~~~~~~
           18: }
check:19'0     ~
           19: 
check:19'0     ~
           20: ; Function Attrs: nounwind readnone uwtable willreturn
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21: define dso_local i64 @cast_f32_i64(float %a) unnamed_addr #0 {
check:19'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:28'0                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           22: start:
check:28'0     ~~~~~~
           23:  %0 = tail call i64 @llvm.fptosi.sat.i64.f32(float %a)
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:28'1           ?                                                possible intended match
           24:  ret i64 %0
check:28'0     ~~~~~~~~~~~
           25: }
check:28'0     ~
           26: 
check:28'0     ~
           27: ; Function Attrs: nounwind readnone uwtable willreturn
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28: define dso_local i32 @cast_f32_i32(float %a) unnamed_addr #0 {
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:37'0                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           29: start:
check:37'0     ~~~~~~
           30:  %0 = tail call i32 @llvm.fptosi.sat.i32.f32(float %a)
check:37'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:37'1           ?                                                possible intended match
           31:  ret i32 %0
check:37'0     ~~~~~~~~~~~
           32: }
check:37'0     ~
           33: 
check:37'0     ~
           34: ; Function Attrs: nounwind readnone uwtable willreturn
check:37'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35: define dso_local i64 @cast_f64_u64(double %a) unnamed_addr #0 {
check:37'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:46'0                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           36: start:
check:46'0     ~~~~~~
           37:  %0 = tail call i64 @llvm.fptoui.sat.i64.f64(double %a)
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:46'1           ?                                                 possible intended match
           38:  ret i64 %0
check:46'0     ~~~~~~~~~~~
           39: }
check:46'0     ~
           40: 
check:46'0     ~
           41: ; Function Attrs: nounwind readnone uwtable willreturn
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42: define dso_local i32 @cast_f64_u32(double %a) unnamed_addr #0 {
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:55'0                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           43: start:
check:55'0     ~~~~~~
           44:  %0 = tail call i32 @llvm.fptoui.sat.i32.f64(double %a)
check:55'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:55'1           ?                                                 possible intended match
           45:  ret i32 %0
check:55'0     ~~~~~~~~~~~
           46: }
check:55'0     ~
           47: 
check:55'0     ~
           48: ; Function Attrs: nounwind readnone uwtable willreturn
check:55'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           49: define dso_local i64 @cast_f32_u64(float %a) unnamed_addr #0 {
check:55'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:64'0                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           50: start:
check:64'0     ~~~~~~
           51:  %0 = tail call i64 @llvm.fptoui.sat.i64.f32(float %a)
check:64'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:64'1           ?                                                possible intended match
           52:  ret i64 %0
check:64'0     ~~~~~~~~~~~
           53: }
check:64'0     ~
           54: 
check:64'0     ~
           55: ; Function Attrs: nounwind readnone uwtable willreturn
check:64'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56: define dso_local i32 @cast_f32_u32(float %a) unnamed_addr #0 {
check:64'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:73'0                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           57: start:
check:73'0     ~~~~~~
           58:  %0 = tail call i32 @llvm.fptoui.sat.i32.f32(float %a)
check:73'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:73'1           ?                                                possible intended match
           59:  ret i32 %0
check:73'0     ~~~~~~~~~~~
           60: }
check:73'0     ~
           61: 
check:73'0     ~
           62: ; Function Attrs: nounwind readnone uwtable willreturn
check:73'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63: define dso_local i8 @cast_f32_u8(float %a) unnamed_addr #0 {
check:73'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:81'0                                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           64: start:
check:81'0     ~~~~~~
           65:  %0 = tail call i8 @llvm.fptoui.sat.i8.f32(float %a)
check:81'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:81'1                                            ?             possible intended match
           66:  ret i8 %0
check:81'0     ~~~~~~~~~~
           67: }
check:81'0     ~
           68: 
check:81'0     ~
           69: ; Function Attrs: nounwind readnone uwtable willreturn
check:81'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70: define dso_local i64 @cast_unchecked_f64_i64(double %a) unnamed_addr #0 {
check:81'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>


------------------------------------------


---- [codegen] codegen/wasm_casts_nontrapping.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll" "/checkout/src/test/codegen/wasm_casts_nontrapping.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:8:12: error: CHECK: expected string not found in input
 // CHECK: tail call i64 @llvm.wasm.trunc.saturate.signed.i64.f64(double {{.*}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:7:35: note: scanning from here
define dso_local i64 @cast_f64_i64(double %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:9:7: note: possible intended match here
 %0 = tail call i64 @llvm.fptosi.sat.i64.f64(double %a)
      ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:16:12: error: CHECK: expected string not found in input
 // CHECK: tail call i32 @llvm.wasm.trunc.saturate.signed.i32.f64(double {{.*}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:14:35: note: scanning from here
define dso_local i32 @cast_f64_i32(double %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:16:7: note: possible intended match here
 %0 = tail call i32 @llvm.fptosi.sat.i32.f64(double %a)
      ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:24:12: error: CHECK: expected string not found in input
 // CHECK: tail call i64 @llvm.wasm.trunc.saturate.signed.i64.f32(float {{.*}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:21:35: note: scanning from here
define dso_local i64 @cast_f32_i64(float %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:23:7: note: possible intended match here
 %0 = tail call i64 @llvm.fptosi.sat.i64.f32(float %a)
      ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:32:12: error: CHECK: expected string not found in input
 // CHECK: tail call i32 @llvm.wasm.trunc.saturate.signed.i32.f32(float {{.*}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:28:35: note: scanning from here
define dso_local i32 @cast_f32_i32(float %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:30:7: note: possible intended match here
 %0 = tail call i32 @llvm.fptosi.sat.i32.f32(float %a)
      ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:41:12: error: CHECK: expected string not found in input
 // CHECK: tail call i64 @llvm.wasm.trunc.saturate.unsigned.i64.f64(double {{.*}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:35:35: note: scanning from here
define dso_local i64 @cast_f64_u64(double %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:37:7: note: possible intended match here
 %0 = tail call i64 @llvm.fptoui.sat.i64.f64(double %a)
      ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:49:12: error: CHECK: expected string not found in input
 // CHECK: tail call i32 @llvm.wasm.trunc.saturate.unsigned.i32.f64(double {{.*}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:42:35: note: scanning from here
define dso_local i32 @cast_f64_u32(double %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:44:7: note: possible intended match here
 %0 = tail call i32 @llvm.fptoui.sat.i32.f64(double %a)
      ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:57:12: error: CHECK: expected string not found in input
 // CHECK: tail call i64 @llvm.wasm.trunc.saturate.unsigned.i64.f32(float {{.*}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:49:35: note: scanning from here
define dso_local i64 @cast_f32_u64(float %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:51:7: note: possible intended match here
 %0 = tail call i64 @llvm.fptoui.sat.i64.f32(float %a)
      ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:65:12: error: CHECK: expected string not found in input
 // CHECK: tail call i32 @llvm.wasm.trunc.saturate.unsigned.i32.f32(float {{.*}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:56:35: note: scanning from here
define dso_local i32 @cast_f32_u32(float %a) unnamed_addr #0 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:58:7: note: possible intended match here
 %0 = tail call i32 @llvm.fptoui.sat.i32.f32(float %a)
      ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:74:12: error: CHECK: expected string not found in input
 // CHECK: fptoui float {{.*}} to i8
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:63:33: note: scanning from here
define dso_local i8 @cast_f32_u8(float %a) unnamed_addr #0 {
                                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:65:40: note: possible intended match here
 %0 = tail call i8 @llvm.fptoui.sat.i8.f32(float %a)
                                       ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:86:12: error: CHECK: expected string not found in input
 // CHECK: fptosi double {{.*}} to i64
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:70:45: note: scanning from here
define dso_local i64 @cast_unchecked_f64_i64(double %a) unnamed_addr #0 {
                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:72:49: note: possible intended match here
 %0 = tail call i64 @llvm.wasm.trunc.signed.i64.f64(double %a) #4
                                                ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:95:12: error: CHECK: expected string not found in input
 // CHECK: fptosi double {{.*}} to i32
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:77:45: note: scanning from here
define dso_local i32 @cast_unchecked_f64_i32(double %a) unnamed_addr #0 {
                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:79:49: note: possible intended match here
 %0 = tail call i32 @llvm.wasm.trunc.signed.i32.f64(double %a) #4
                                                ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:104:12: error: CHECK: expected string not found in input
 // CHECK: fptosi float {{.*}} to i64
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:84:45: note: scanning from here
define dso_local i64 @cast_unchecked_f32_i64(float %a) unnamed_addr #0 {
                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:86:49: note: possible intended match here
 %0 = tail call i64 @llvm.wasm.trunc.signed.i64.f32(float %a) #4
                                                ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:113:12: error: CHECK: expected string not found in input
 // CHECK: fptosi float {{.*}} to i32
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:91:45: note: scanning from here
define dso_local i32 @cast_unchecked_f32_i32(float %a) unnamed_addr #0 {
                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:93:49: note: possible intended match here
 %0 = tail call i32 @llvm.wasm.trunc.signed.i32.f32(float %a) #4
                                                ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:123:12: error: CHECK: expected string not found in input
 // CHECK: fptoui double {{.*}} to i64
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:98:45: note: scanning from here
define dso_local i64 @cast_unchecked_f64_u64(double %a) unnamed_addr #0 {
                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:100:51: note: possible intended match here
 %0 = tail call i64 @llvm.wasm.trunc.unsigned.i64.f64(double %a) #4
                                                  ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:132:12: error: CHECK: expected string not found in input
 // CHECK: fptoui double {{.*}} to i32
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:105:45: note: scanning from here
define dso_local i32 @cast_unchecked_f64_u32(double %a) unnamed_addr #0 {
                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:107:51: note: possible intended match here
 %0 = tail call i32 @llvm.wasm.trunc.unsigned.i32.f64(double %a) #4
                                                  ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:141:12: error: CHECK: expected string not found in input
 // CHECK: fptoui float {{.*}} to i64
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:112:45: note: scanning from here
define dso_local i64 @cast_unchecked_f32_u64(float %a) unnamed_addr #0 {
                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:114:51: note: possible intended match here
 %0 = tail call i64 @llvm.wasm.trunc.unsigned.i64.f32(float %a) #4
                                                  ^
/checkout/src/test/codegen/wasm_casts_nontrapping.rs:150:12: error: CHECK: expected string not found in input
 // CHECK: fptoui float {{.*}} to i32
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:119:45: note: scanning from here
define dso_local i32 @cast_unchecked_f32_u32(float %a) unnamed_addr #0 {
                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll:121:51: note: possible intended match here
 %0 = tail call i32 @llvm.wasm.trunc.unsigned.i32.f32(float %a) #4

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/wasm_casts_nontrapping/wasm_casts_nontrapping.ll
Check file: /checkout/src/test/codegen/wasm_casts_nontrapping.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
             1: ; ModuleID = 'wasm_casts_nontrapping.3a1fbbbh-cgu.0'
             2: source_filename = "wasm_casts_nontrapping.3a1fbbbh-cgu.0"
             3: target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
             4: target triple = "wasm32-unknown-emscripten"
             5: 
             6: ; Function Attrs: nounwind readnone uwtable willreturn
             7: define dso_local i64 @cast_f64_i64(double %a) unnamed_addr #0 {
check:8'0                                         X~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
             8: start:
check:8'0       ~~~~~~
             9:  %0 = tail call i64 @llvm.fptosi.sat.i64.f64(double %a)
check:8'0       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:8'1             ?                                                 possible intended match
            10:  ret i64 %0
check:8'0       ~~~~~~~~~~~
            11: }
check:8'0       ~
            12: 
check:8'0       ~
            13: ; Function Attrs: nounwind readnone uwtable willreturn
check:8'0       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            14: define dso_local i32 @cast_f64_i32(double %a) unnamed_addr #0 {
check:8'0       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'0                                        X~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            15: start:
check:16'0      ~~~~~~
            16:  %0 = tail call i32 @llvm.fptosi.sat.i32.f64(double %a)
check:16'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'1            ?                                                 possible intended match
            17:  ret i32 %0
check:16'0      ~~~~~~~~~~~
            18: }
check:16'0      ~
            19: 
check:16'0      ~
            20: ; Function Attrs: nounwind readnone uwtable willreturn
check:16'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            21: define dso_local i64 @cast_f32_i64(float %a) unnamed_addr #0 {
check:16'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:24'0                                        X~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            22: start:
check:24'0      ~~~~~~
            23:  %0 = tail call i64 @llvm.fptosi.sat.i64.f32(float %a)
check:24'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:24'1            ?                                                possible intended match
            24:  ret i64 %0
check:24'0      ~~~~~~~~~~~
            25: }
check:24'0      ~
            26: 
check:24'0      ~
            27: ; Function Attrs: nounwind readnone uwtable willreturn
check:24'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            28: define dso_local i32 @cast_f32_i32(float %a) unnamed_addr #0 {
check:24'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:32'0                                        X~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            29: start:
check:32'0      ~~~~~~
            30:  %0 = tail call i32 @llvm.fptosi.sat.i32.f32(float %a)
check:32'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:32'1            ?                                                possible intended match
            31:  ret i32 %0
check:32'0      ~~~~~~~~~~~
            32: }
check:32'0      ~
            33: 
check:32'0      ~
---
test result: FAILED. 194 passed; 2 failed; 75 ignored; 0 measured; 0 filtered out; finished in 1.49s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-wasm32-unknown-emscripten" "--suite" "codegen" "--mode" "codegen" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/latest/bin/node" "--npm" "/emsdk-portable/node/latest/bin/npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --host= --target wasm32-unknown-emscripten --exclude library/alloc
Build completed unsuccessfully in 0:25:47
