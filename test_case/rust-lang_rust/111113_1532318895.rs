plain
 finished in 0.371 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 155 tests
iii..............iiiiiiiiiiiiiF.........................................................  88/155

failures:

---- [assembly] tests/assembly/align_offset.rs stdout ----
---- [assembly] tests/assembly/align_offset.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s" "/checkout/tests/assembly/align_offset.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/assembly/align_offset.rs:31:11: error: CHECK: expected string not found in input
/checkout/tests/assembly/align_offset.rs:31:11: error: CHECK: expected string not found in input
// CHECK: orq
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:144:6: note: scanning from here
 shrq $2, %rax
     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s:147:22: note: possible intended match here
 .size align_offset_word_ptr, .Lfunc_end4-align_offset_word_ptr

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/align_offset/align_offset.s
Check file: /checkout/tests/assembly/align_offset.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           44:  subq %rdi, %rcx 
           45:  movq %r9, %rax 
           46:  shrq %rax 
           47:  leaq .L__unnamed_1(%rip), %rdx 
           48:  movzbl (%rax,%rdx), %eax 
           49:  cmpq $17, %rsi 
           50:  jb .LBB0_9 
           51:  movq %r9, %rdi 
           52:  imulq %rax, %rdi 
           53:  movl $2, %r10d 
           54:  movl $2, %edx 
           55:  subq %rdi, %rdx 
           56:  imulq %rdx, %rax 
           57:  cmpq $257, %rsi 
           58:  jb .LBB0_9 
           59:  movq %rax, %rdx 
           60:  imulq %r9, %rdx 
           61:  subq %rdx, %r10 
           62:  imulq %r10, %rax 
           63:  cmpq $65537, %rsi 
           64:  jb .LBB0_9 
           65:  movq %rax, %rdi 
           66:  imulq %r9, %rdi 
           67:  movl $2, %r10d 
           68:  movl $2, %edx 
           69:  subq %rdi, %rdx 
           70:  imulq %rdx, %rax 
           71:  movabsq $4294967297, %rdx 
           72:  cmpq %rdx, %rsi 
           73:  jb .LBB0_9 
           74:  imulq %rax, %r9 
           75:  subq %r9, %r10 
           76:  imulq %r10, %rax 
           77: .LBB0_9: 
           78:  andq %r8, %rax 
           79:  imulq %rcx, %rax 
           80:  andq %r8, %rax 
           81:  retq 
           82: .Lfunc_end0: 
           83:  .size _ZN4core3ptr12align_offset17h663c57cfe37f2997E, .Lfunc_end0-_ZN4core3ptr12align_offset17h663c57cfe37f2997E 
           84:  .cfi_endproc 
           85:  
           86:  .section .text._ZN4core3ptr12align_offset17hdc1f1e619dd1edd9E,"ax",@progbits 
           87:  .globl _ZN4core3ptr12align_offset17hdc1f1e619dd1edd9E 
           88:  .p2align 4, 0x90 
           89:  .type _ZN4core3ptr12align_offset17hdc1f1e619dd1edd9E,@function 
           90: _ZN4core3ptr12align_offset17hdc1f1e619dd1edd9E: 
           91:  .cfi_startproc 
           92:  leaq (%rsi,%rdi), %rax 
           93:  addq $-1, %rax 
           94:  negq %rsi 
           95:  andq %rsi, %rax 
           96:  subq %rdi, %rax 
           97:  retq 
           98: .Lfunc_end1: 
           99:  .size _ZN4core3ptr12align_offset17hdc1f1e619dd1edd9E, .Lfunc_end1-_ZN4core3ptr12align_offset17hdc1f1e619dd1edd9E 
          100:  .cfi_endproc 
          101:  
          102:  .section .text.align_offset_byte_ptr,"ax",@progbits 
          103:  .globl align_offset_byte_ptr 
          104:  .p2align 4, 0x90 
          105:  .type align_offset_byte_ptr,@function 
          106: align_offset_byte_ptr: 
          107:  .cfi_startproc 
          108:  leaq 31(%rdi), %rax 
          109:  andq $-32, %rax 
          110:  subq %rdi, %rax 
          111:  retq 
          112: .Lfunc_end2: 
          113:  .size align_offset_byte_ptr, .Lfunc_end2-align_offset_byte_ptr 
          114:  .cfi_endproc 
          115:  
          116:  .section .text.align_offset_byte_slice,"ax",@progbits 
          117:  .globl align_offset_byte_slice 
          118:  .p2align 4, 0x90 
          119:  .type align_offset_byte_slice,@function 
          121:  .cfi_startproc 
          121:  .cfi_startproc 
          122:  leaq 31(%rdi), %rax 
          123:  andq $-32, %rax 
          124:  subq %rdi, %rax 
          125:  retq 
          126: .Lfunc_end3: 
          127:  .size align_offset_byte_slice, .Lfunc_end3-align_offset_byte_slice 
          128:  .cfi_endproc 
          129:  
          130:  .section .text.align_offset_word_ptr,"ax",@progbits 
          131:  .globl align_offset_word_ptr 
          132:  .p2align 4, 0x90 
          133:  .type align_offset_word_ptr,@function 
          134: align_offset_word_ptr: 
          135:  .cfi_startproc 
          136:  movq $-1, %rax 
          137:  testb $3, %dil 
          138:  je .LBB4_1 
          139:  retq 
          140: .LBB4_1: 
          141:  leaq 31(%rdi), %rax 
          142:  andq $-32, %rax 
          143:  subq %rdi, %rax 
          144:  shrq $2, %rax 
check:31'0          X~~~~~~~~~ error: no match found
          145:  retq 
check:31'0     ~~~~~~
          146: .Lfunc_end4: 
check:31'0     ~~~~~~~~~~~~~
          147:  .size align_offset_word_ptr, .Lfunc_end4-align_offset_word_ptr 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'1                          ?                                           possible intended match
          148:  .cfi_endproc 
check:31'0     ~~~~~~~~~~~~~~
          149:  
check:31'0     ~
          150:  .section .text.align_offset_word_slice,"ax",@progbits 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          151:  .globl align_offset_word_slice 
          152:  .p2align 4, 0x90 
          153:  .type align_offset_word_slice,@function 
          154: align_offset_word_slice: 
          155:  .cfi_startproc 
          156:  leaq 31(%rdi), %rax 
          157:  andq $-32, %rax 
          158:  subq %rdi, %rax 
          159:  shrq $2, %rax 
          160:  retq 
          161: .Lfunc_end5: 
          162:  .size align_offset_word_slice, .Lfunc_end5-align_offset_word_slice 
          163:  .cfi_endproc 
          164:  
          165:  .type .L__unnamed_1,@object 
          166:  .section .rodata.cst8,"aM",@progbits,8 
          167: .L__unnamed_1: 
          168:  .ascii "\001\013\r\007\t\003\005\017" 
          169:  .size .L__unnamed_1, 8 
          170:  
          171:  .section ".note.GNU-stack","",@progbits 
------------------------------------------



