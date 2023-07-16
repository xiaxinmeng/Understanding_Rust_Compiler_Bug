 diff
--- test2.s 2014-08-09 02:30:46.000000000 -0400
+++ test3.s 2014-08-09 02:30:20.000000000 -0400
@@ -1,6 +1,6 @@
    .section    __TEXT,__text,regular,pure_instructions
    .align  4, 0x90
-__ZN4main20h8b95006aaacf56a1eaaE:
+__ZN4main20h503ced65c10fa3edeaaE:
    .cfi_startproc
    cmpq    %gs:816, %rsp
    ja  LBB0_2
@@ -18,7 +18,6 @@
 Ltmp2:
    .cfi_def_cfa_register %rbp
    ## InlineAsm Start
-test:
    ## InlineAsm End
    popq    %rbp
    retq
@@ -45,7 +44,7 @@
    .cfi_def_cfa_register %rbp
    movq    %rsi, %rax
    movq    %rdi, %rcx
-   leaq    __ZN4main20h8b95006aaacf56a1eaaE(%rip), %rdi
+   leaq    __ZN4main20h503ced65c10fa3edeaaE(%rip), %rdi
    movq    %rcx, %rsi
    movq    %rax, %rdx
    popq    %rbp
