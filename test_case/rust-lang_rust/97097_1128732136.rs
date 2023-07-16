plain
failures:

---- [codegen] src/test/codegen/asm-clobber_abi.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll" "/checkout/src/test/codegen/asm-clobber_abi.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/asm-clobber_abi.rs:9:11: error: CHECK: expected string not found in input
// CHECK: ={ax},={cx},={dx},={si},={di},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{dirflag},~{fpsr},~{flags},~{memory}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll:7:28: note: scanning from here
define void @clobber_sysv64() unnamed_addr #0 {
                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll:9:217: note: possible intended match here
 %0 = tail call { i32, i32, i32, i32, i32, i32, i32, i32, i32, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float } asm sideeffect alignstack inteldialect "", "={ax},={cx},={dx},={si},={di},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{tmm0},~{tmm1},~{tmm2},~{tmm3},~{tmm4},~{tmm5},~{tmm6},~{tmm7},~{dirflag},~{fpsr},~{flags},~{memory}"() #1, !srcloc !2
                                                                                                                                                                                                                        ^
/checkout/src/test/codegen/asm-clobber_abi.rs:16:11: error: CHECK: expected string not found in input
// CHECK: ={ax},={cx},={dx},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{dirflag},~{fpsr},~{flags},~{memory}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll:14:27: note: scanning from here
define void @clobber_win64() unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll:16:207: note: possible intended match here
 %0 = tail call { i32, i32, i32, i32, i32, i32, i32, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float } asm sideeffect alignstack inteldialect "", "={ax},={cx},={dx},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{tmm0},~{tmm1},~{tmm2},~{tmm3},~{tmm4},~{tmm5},~{tmm6},~{tmm7},~{dirflag},~{fpsr},~{flags},~{memory}"() #1, !srcloc !3
                                                                                                                                                                                                              ^
/checkout/src/test/codegen/asm-clobber_abi.rs:23:11: error: CHECK: expected string not found in input
// CHECK: =&{dx},={ax},={cx},={si},={di},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{dirflag},~{fpsr},~{flags},~{memory}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll:21:28: note: scanning from here
define void @clobber_sysv64_edx() unnamed_addr #0 {
                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll:23:217: note: possible intended match here
 %0 = tail call { i32, i32, i32, i32, i32, i32, i32, i32, i32, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float } asm sideeffect alignstack inteldialect "", "=&{dx},={ax},={cx},={si},={di},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{tmm0},~{tmm1},~{tmm2},~{tmm3},~{tmm4},~{tmm5},~{tmm6},~{tmm7},~{dirflag},~{fpsr},~{flags},~{memory}"() #1, !srcloc !4
                                                                                                                                                                                                                        ^
/checkout/src/test/codegen/asm-clobber_abi.rs:31:11: error: CHECK: expected string not found in input
// CHECK: =&{dx},={ax},={cx},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{dirflag},~{fpsr},~{flags},~{memory}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll:28:27: note: scanning from here
define void @clobber_win64_edx() unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll:30:207: note: possible intended match here
 %0 = tail call { i32, i32, i32, i32, i32, i32, i32, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float } asm sideeffect alignstack inteldialect "", "=&{dx},={ax},={cx},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{tmm0},~{tmm1},~{tmm2},~{tmm3},~{tmm4},~{tmm5},~{tmm6},~{tmm7},~{dirflag},~{fpsr},~{flags},~{memory}"() #1, !srcloc !5

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/asm-clobber_abi/asm-clobber_abi.ll
Check file: /checkout/src/test/codegen/asm-clobber_abi.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'asm_clobber_abi.1bec3be4-cgu.0'
            2: source_filename = "asm_clobber_abi.1bec3be4-cgu.0"
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            4: target triple = "x86_64-unknown-linux-gnu"
            5: 
            6: ; Function Attrs: nounwind nonlazybind uwtable
            7: define void @clobber_sysv64() unnamed_addr #0 {
check:9'0                                 X~~~~~~~~~~~~~~~~~~~ error: no match found
            8: start:
check:9'0      ~~~~~~
            9:  %0 = tail call { i32, i32, i32, i32, i32, i32, i32, i32, i32, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float } asm sideeffect alignstack inteldialect "", "={ax},={cx},={dx},={si},={di},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{tmm0},~{tmm1},~{tmm2},~{tmm3},~{tmm4},~{tmm5},~{tmm6},~{tmm7},~{dirflag},~{fpsr},~{flags},~{memory}"() #1, !srcloc !2
check:9'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:9'1                                                                                                                                                                                                                              ?                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              possible intended match
           10:  ret void
check:9'0      ~~~~~~~~~
           11: }
check:9'0      ~
           12: 
check:9'0      ~
           13: ; Function Attrs: nounwind nonlazybind uwtable
check:9'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14: define void @clobber_win64() unnamed_addr #0 {
check:9'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'0                               X~~~~~~~~~~~~~~~~~~~ error: no match found
           15: start:
check:16'0     ~~~~~~
           16:  %0 = tail call { i32, i32, i32, i32, i32, i32, i32, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float } asm sideeffect alignstack inteldialect "", "={ax},={cx},={dx},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{tmm0},~{tmm1},~{tmm2},~{tmm3},~{tmm4},~{tmm5},~{tmm6},~{tmm7},~{dirflag},~{fpsr},~{flags},~{memory}"() #1, !srcloc !3
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'1                                                                                                                                                                                                                   ?                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  possible intended match
           17:  ret void
check:16'0     ~~~~~~~~~
           18: }
check:16'0     ~
           19: 
check:16'0     ~
           20: ; Function Attrs: nounwind nonlazybind uwtable
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21: define void @clobber_sysv64_edx() unnamed_addr #0 {
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:23'0                                X~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           22: start:
check:23'0     ~~~~~~
           23:  %0 = tail call { i32, i32, i32, i32, i32, i32, i32, i32, i32, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float } asm sideeffect alignstack inteldialect "", "=&{dx},={ax},={cx},={si},={di},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{tmm0},~{tmm1},~{tmm2},~{tmm3},~{tmm4},~{tmm5},~{tmm6},~{tmm7},~{dirflag},~{fpsr},~{flags},~{memory}"() #1, !srcloc !4
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:23'1                                                                                                                                                                                                                             ?                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               possible intended match
           24:  ret void
check:23'0     ~~~~~~~~~
           25: }
check:23'0     ~
           26: 
check:23'0     ~
           27: ; Function Attrs: nounwind nonlazybind uwtable
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28: define void @clobber_win64_edx() unnamed_addr #0 {
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'0                               X~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           29: start:
check:31'0     ~~~~~~
           30:  %0 = tail call { i32, i32, i32, i32, i32, i32, i32, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float } asm sideeffect alignstack inteldialect "", "=&{dx},={ax},={cx},={r8},={r9},={r10},={r11},={xmm0},={xmm1},={xmm2},={xmm3},={xmm4},={xmm5},={xmm6},={xmm7},={xmm8},={xmm9},={xmm10},={xmm11},={xmm12},={xmm13},={xmm14},={xmm15},~{xmm16},~{xmm17},~{xmm18},~{xmm19},~{xmm20},~{xmm21},~{xmm22},~{xmm23},~{xmm24},~{xmm25},~{xmm26},~{xmm27},~{xmm28},~{xmm29},~{xmm30},~{xmm31},~{k0},~{k1},~{k2},~{k3},~{k4},~{k5},~{k6},~{k7},~{st},~{st(1)},~{st(2)},~{st(3)},~{st(4)},~{st(5)},~{st(6)},~{st(7)},~{tmm0},~{tmm1},~{tmm2},~{tmm3},~{tmm4},~{tmm5},~{tmm6},~{tmm7},~{dirflag},~{fpsr},~{flags},~{memory}"() #1, !srcloc !5
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'1                                                                                                                                                                                                                   ?                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   possible intended match
           31:  ret void
check:31'0     ~~~~~~~~~
           32: }
check:31'0     ~
           33: 
check:31'0     ~
           34: attributes #0 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35: attributes #1 = { nounwind }
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
