plain
failures:

---- [assembly] tests/assembly/x86_64-floating-point-clamp.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s" "/checkout/tests/assembly/x86_64-floating-point-clamp.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/assembly/x86_64-floating-point-clamp.rs:12:12: error: CHECK: expected string not found in input
/checkout/tests/assembly/x86_64-floating-point-clamp.rs:12:12: error: CHECK: expected string not found in input
 // CHECK: maxss
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:8:12: note: scanning from here
clamp_demo:
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:12:2: note: possible intended match here
 movss dword ptr [rsp], xmm1
 ^
/checkout/tests/assembly/x86_64-floating-point-clamp.rs:20:12: error: CHECK: expected string not found in input
/checkout/tests/assembly/x86_64-floating-point-clamp.rs:20:12: error: CHECK: expected string not found in input
 // CHECK: movss xmm1
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:65:14: note: scanning from here
clamp12_demo:
             ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s:68:2: note: possible intended match here
 movss xmm0, dword ptr [rip + .LCPI1_0]

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/x86_64-floating-point-clamp/x86_64-floating-point-clamp.s
Check file: /checkout/tests/assembly/x86_64-floating-point-clamp.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            1:  .text 
            1:  .text 
            2:  .intel_syntax noprefix 
            3:  .file "x86_64_floating_point_clamp.1f7dbb53b27604f7-cgu.0" 
            4:  .section .text.clamp_demo,"ax",@progbits 
            5:  .globl clamp_demo 
            6:  .p2align 4, 0x90 
            7:  .type clamp_demo,@function 
            8: clamp_demo: 
check:12'0                X error: no match found
            9:  .cfi_startproc 
check:12'0     ~~~~~~~~~~~~~~~~
           10:  sub rsp, 88 
check:12'0     ~~~~~~~~~~~~~
           11:  .cfi_def_cfa_offset 96 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           12:  movss dword ptr [rsp], xmm1 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:12'1      ?                            possible intended match
           13:  movss dword ptr [rsp + 4], xmm2 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  ucomiss xmm2, xmm1 
check:12'0     ~~~~~~~~~~~~~~~~~~~~
           15:  jb .LBB0_6 
check:12'0     ~~~~~~~~~~~~
           16:  ucomiss xmm1, xmm0 
check:12'0     ~~~~~~~~~~~~~~~~~~~~
           17:  jbe .LBB0_2 
check:12'0     ~~~~~~~~~~~~~
           18:  ucomiss xmm1, xmm2 
check:12'0     ~~~~~~~~~~~~~~~~~~~~
           19:  movaps xmm0, xmm1 
check:12'0     ~~~~~~~~~~~~~~~~~~~
           20:  ja .LBB0_4 
check:12'0     ~~~~~~~~~~~~
           21:  jmp .LBB0_5 
check:12'0     ~~~~~~~~~~~~~
           22: .LBB0_2: 
check:12'0     ~~~~~~~~~
           23:  ucomiss xmm0, xmm2 
check:12'0     ~~~~~~~~~~~~~~~~~~~~
           24:  jbe .LBB0_5 
check:12'0     ~~~~~~~~~~~~~
           25: .LBB0_4: 
check:12'0     ~~~~~~~~~
           26:  movaps xmm0, xmm2 
check:12'0     ~~~~~~~~~~~~~~~~~~~
           27: .LBB0_5: 
check:12'0     ~~~~~~~~~
           28:  add rsp, 88 
check:12'0     ~~~~~~~~~~~~~
           29:  .cfi_def_cfa_offset 8 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~
           30:  ret 
check:12'0     ~~~~~
           31: .LBB0_6: 
check:12'0     ~~~~~~~~~
           32:  .cfi_def_cfa_offset 96 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           33:  mov rax, rsp 
check:12'0     ~~~~~~~~~~~~~~
           34:  mov qword ptr [rsp + 8], rax 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35:  mov rax, qword ptr [rip + _ZN4core3fmt5float50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$f32$GT$3fmt17h8d56cc4305a14123E@GOTPCREL] 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36:  mov qword ptr [rsp + 16], rax 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37:  lea rcx, [rsp + 4] 
check:12'0     ~~~~~~~~~~~~~~~~~~~~
           38:  mov qword ptr [rsp + 24], rcx 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39:  mov qword ptr [rsp + 32], rax 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  lea rax, [rip + .L__unnamed_1] 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  mov qword ptr [rsp + 40], rax 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  mov qword ptr [rsp + 48], 2 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  mov qword ptr [rsp + 72], 0 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  lea rax, [rsp + 8] 
check:12'0     ~~~~~~~~~~~~~~~~~~~~
           45:  mov qword ptr [rsp + 56], rax 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  mov qword ptr [rsp + 64], 2 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47:  lea rsi, [rip + .L__unnamed_2] 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48:  lea rdi, [rsp + 40] 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~
           49:  call qword ptr [rip + _ZN4core9panicking9panic_fmt17h7563e3de82c128b3E@GOTPCREL] 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50:  ud2 
check:12'0     ~~~~~
           51: .Lfunc_end0: 
check:12'0     ~~~~~~~~~~~~~
           52:  .size clamp_demo, .Lfunc_end0-clamp_demo 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  .cfi_endproc 
check:12'0     ~~~~~~~~~~~~~~
           54:  
check:12'0     ~
           55:  .section .rodata.cst4,"aM",@progbits,4 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56:  .p2align 2 
check:12'0     ~~~~~~~~~~~~
           57: .LCPI1_0: 
check:12'0     ~~~~~~~~~~
           58:  .long 0x3f800000 
check:12'0     ~~~~~~~~~~~~~~~~~~
           59: .LCPI1_1: 
check:12'0     ~~~~~~~~~~
           60:  .long 0x40000000 
check:12'0     ~~~~~~~~~~~~~~~~~~
           61:  .section .text.clamp12_demo,"ax",@progbits 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62:  .globl clamp12_demo 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~
           63:  .p2align 4, 0x90 
check:12'0     ~~~~~~~~~~~~~~~~~~
           64:  .type clamp12_demo,@function 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65: clamp12_demo: 
check:12'0     ~~~~~~~~~~~~~
check:20'0                  X error: no match found
           66:  .cfi_startproc 
check:20'0     ~~~~~~~~~~~~~~~~
           67:  movaps xmm1, xmm0 
check:20'0     ~~~~~~~~~~~~~~~~~~~
           68:  movss xmm0, dword ptr [rip + .LCPI1_0] 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:20'1      ?                                       possible intended match
           69:  ucomiss xmm0, xmm1 
check:20'0     ~~~~~~~~~~~~~~~~~~~~
           70:  ja .LBB1_3 
check:20'0     ~~~~~~~~~~~~
           71:  ucomiss xmm1, dword ptr [rip + .LCPI1_1] 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72:  movaps xmm0, xmm1 
check:20'0     ~~~~~~~~~~~~~~~~~~~
           73:  jbe .LBB1_3 
check:20'0     ~~~~~~~~~~~~~
           74:  movss xmm0, dword ptr [rip + .LCPI1_1] 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           75: .LBB1_3: 
check:20'0     ~~~~~~~~~
           76:  ret 
check:20'0     ~~~~~
           77: .Lfunc_end1: 
check:20'0     ~~~~~~~~~~~~~
           78:  .size clamp12_demo, .Lfunc_end1-clamp12_demo 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79:  .cfi_endproc 
check:20'0     ~~~~~~~~~~~~~~
           80:  
check:20'0     ~
           81:  .type .L__unnamed_3,@object 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           82:  .section .rodata..L__unnamed_3,"a",@progbits 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           83: .L__unnamed_3: 
check:20'0     ~~~~~~~~~~~~~~~
           84:  .ascii "min > max, or either was NaN. min = " 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           85:  .size .L__unnamed_3, 36 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           86:  
check:20'0     ~
           87:  .type .L__unnamed_4,@object 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           88:  .section .rodata.cst8,"aM",@progbits,8 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           89: .L__unnamed_4: 
check:20'0     ~~~~~~~~~~~~~~~
           90:  .ascii ", max = " 
check:20'0     ~~~~~~~~~~~~~~~~~~~
           91:  .size .L__unnamed_4, 8 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           92:  
check:20'0     ~
           93:  .type .L__unnamed_1,@object 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           94:  .section .data.rel.ro..L__unnamed_1,"aw",@progbits 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           95:  .p2align 3 
check:20'0     ~~~~~~~~~~~~
           96: .L__unnamed_1: 
check:20'0     ~~~~~~~~~~~~~~~
           97:  .quad .L__unnamed_3 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~
           98:  .asciz "$\000\000\000\000\000\000" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           99:  .quad .L__unnamed_4 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~
          100:  .asciz "\b\000\000\000\000\000\000" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          101:  .size .L__unnamed_1, 32 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          102:  
check:20'0     ~
          103:  .type .L__unnamed_5,@object 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          104:  .section .rodata..L__unnamed_5,"a",@progbits 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          105: .L__unnamed_5: 
check:20'0     ~~~~~~~~~~~~~~~
          106:  .ascii "/checkout/library/core/src/num/f32.rs" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          107:  .size .L__unnamed_5, 37 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          108:  
check:20'0     ~
          109:  .type .L__unnamed_2,@object 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          110:  .section .data.rel.ro..L__unnamed_2,"aw",@progbits 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          111:  .p2align 3 
check:20'0     ~~~~~~~~~~~~
          112: .L__unnamed_2: 
check:20'0     ~~~~~~~~~~~~~~~
          113:  .quad .L__unnamed_5 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~
          114:  .asciz "%\000\000\000\000\000\000\000\226\005\000\000\t\000\000" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          115:  .size .L__unnamed_2, 24 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          116:  
check:20'0     ~
          117:  .section ".note.GNU-stack","",@progbits 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



