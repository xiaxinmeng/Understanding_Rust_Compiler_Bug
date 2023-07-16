 diff
--- before.s    2014-02-02 23:33:09.706317828 +0100
+++ after.s 2014-02-02 23:33:14.650273534 +0100
@@ -1,51 +1,40 @@
-   .globl  _ZN3foo19h81a28b9b9b1b3681ad4v0.0E
+   .globl  _ZN3foo19h81ff6a854ff46f5cad4v0.0E
    .align  16, 0x90
-   .type   _ZN3foo19h81a28b9b9b1b3681ad4v0.0E,@function
-_ZN3foo19h81a28b9b9b1b3681ad4v0.0E:
+   .type   _ZN3foo19h81ff6a854ff46f5cad4v0.0E,@function
+_ZN3foo19h81ff6a854ff46f5cad4v0.0E:
    .cfi_startproc
    cmpq    %fs:112, %rsp
    ja  .LBB0_2
-   movabsq $120, %r10
+   movabsq $56, %r10
    movabsq $0, %r11
    callq   __morestack
    retq
 .LBB0_2:
-   subq    $120, %rsp
+   subq    $56, %rsp
 .Ltmp1:
-   .cfi_def_cfa_offset 128
+   .cfi_def_cfa_offset 64
    movzbl  %dil, %eax
    cmpl    $1, %eax
    jne .LBB0_4
-   movq    $5, 112(%rsp)
-   leaq    _ZN3fmt11secret_show19h1cfae34b462b5821a24v0.0E(%rip), %rax
-   movq    %rax, 96(%rsp)
-   leaq    112(%rsp), %rax
-   movq    %rax, 104(%rsp)
-   leaq    _ZN3foo15__STATIC_FMTSTR19h1e2ce5d40ae038c7ah4v0.0E(%rip), %rax
-   movq    %rax, 64(%rsp)
-   movq    $2, 72(%rsp)
-   leaq    96(%rsp), %rax
-   movq    %rax, 80(%rsp)
-   movq    $1, 88(%rsp)
-   leaq    64(%rsp), %rdi
+   movq    $5, 48(%rsp)
    jmp .LBB0_5
 .LBB0_4:
-   movq    $6, 56(%rsp)
-   leaq    _ZN3fmt11secret_show19h1cfae34b462b5821a24v0.0E(%rip), %rax
-   movq    %rax, 40(%rsp)
-   leaq    56(%rsp), %rax
-   movq    %rax, 48(%rsp)
-   leaq    _ZN3foo15__STATIC_FMTSTR19h1e2ce5d40ae038c7ah4v0.0E(%rip), %rax
-   movq    %rax, 8(%rsp)
-   movq    $2, 16(%rsp)
-   leaq    40(%rsp), %rax
-   movq    %rax, 24(%rsp)
-   movq    $1, 32(%rsp)
-   leaq    8(%rsp), %rdi
+   movq    $6, 48(%rsp)
 .LBB0_5:
+   leaq    _ZN3fmt11secret_show19h49d00ba23b26f207a24v0.0E(%rip), %rax
+   movq    %rax, 32(%rsp)
+   leaq    48(%rsp), %rax
+   movq    %rax, 40(%rsp)
+   leaq    _ZN3foo15__STATIC_FMTSTR19h96bb7cc571f17624ah4v0.0E(%rip), %rax
+   movq    %rax, (%rsp)
+   movq    $2, 8(%rsp)
+   leaq    32(%rsp), %rax
+   movq    %rax, 16(%rsp)
+   movq    $1, 24(%rsp)
+   leaq    (%rsp), %rdi
    callq   _ZN2io5stdio12println_args19h7932d545acb66ab6ak9v0.10.preE@PLT
-   addq    $120, %rsp
+   addq    $56, %rsp
    retq
 .Ltmp2:
-   .size   _ZN3foo19h81a28b9b9b1b3681ad4v0.0E, .Ltmp2-_ZN3foo19h81a28b9b9b1b3681ad4v0.0E
+   .size   _ZN3foo19h81ff6a854ff46f5cad4v0.0E, .Ltmp2-_ZN3foo19h81ff6a854ff46f5cad4v0.0E
    .cfi_endproc
