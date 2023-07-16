 diff
--- test2.s 2014-08-09 02:17:59.000000000 -0400
+++ test3.s 2014-08-09 02:23:10.000000000 -0400
@@ -13,9 +13,9 @@
 Ldebug_range:
    .section    __TEXT,__text,regular,pure_instructions
    .align  4, 0x90
-__ZN4main20h8b95006aaacf56a1eaaE:
+__ZN4main20h503ced65c10fa3edeaaE:
 Lfunc_begin0:
-   .file   1 "test2.rs"
+   .file   1 "test3.rs"
    .loc    1 3 0
    .cfi_startproc
    cmpq    %gs:816, %rsp
@@ -36,7 +36,6 @@
    .loc    1 5 0 prologue_end
 Ltmp3:
    ## InlineAsm Start
-test:
    ## InlineAsm End
    popq    %rbp
    retq
@@ -64,7 +63,7 @@
 Ltmp7:
    .cfi_def_cfa_register %rbp
    subq    $16, %rsp
-   leaq    __ZN4main20h8b95006aaacf56a1eaaE(%rip), %rax
+   leaq    __ZN4main20h503ced65c10fa3edeaaE(%rip), %rax
    movq    %rdi, -8(%rbp)
    movq    %rax, %rdi
    movq    -8(%rbp), %rax
@@ -82,15 +81,15 @@
 Linfo_string0:
    .asciz  "rustc version 0.12.0-pre-nightly (a1429bca5 2014-08-08 21:36:11 +0000)"
 Linfo_string1:
-   .asciz  "./test2.rs"
+   .asciz  "./test3.rs"
 Linfo_string2:
    .asciz  "/Users/enix/code/rusttest"
 Linfo_string3:
-   .asciz  "test2"
+   .asciz  "test3"
 Linfo_string4:
    .asciz  "main"
 Linfo_string5:
-   .asciz  "_ZN5test24mainE"
+   .asciz  "_ZN5test34mainE"
    .section    __DWARF,__debug_info,regular,debug
 L__DWARF__debug_info_begin0:
    .long   75
@@ -193,9 +192,9 @@
    .short  1
    .short  6
    .long   0
-   .long   1
+   .long   -1
    .long   2090499946
-   .long   1217565201
+   .long   -1785934126
    .long   LNames0-Lnames_begin
    .long   LNames1-Lnames_begin
 LNames0:
@@ -236,7 +235,7 @@
    .short  1
    .short  6
    .long   0
-   .long   275477815
+   .long   275477816
    .long   Lnamespac0-Lnamespac_begin
 Lnamespac0:
 Lset11 = Linfo_string3-Linfo_string
