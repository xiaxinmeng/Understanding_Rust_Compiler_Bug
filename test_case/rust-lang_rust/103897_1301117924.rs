plain
Successfully built 36f8da3a32ea
Successfully tagged rust-ci:latest
Built container sha256:36f8da3a32ea9d041790db9f4ecadb41c90ad438c8881328d1cc808483d01c19
Uploading finished image to https://ci-caches.rust-lang.org/docker/5df1423b252af8fe6e1c1d4bfe9ecee415e5079d069fe81f116b5415ff8f3fff1d8d7f6f046a60e231d4eebdb0071aabb1a1f0159ce874f5e50913c8f8929e6e
upload failed: - to s3://rust-lang-ci-sccache2/docker/5df1423b252af8fe6e1c1d4bfe9ecee415e5079d069fe81f116b5415ff8f3fff1d8d7f6f046a60e231d4eebdb0071aabb1a1f0159ce874f5e50913c8f8929e6e Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
failures:

---- [assembly] src/test/assembly/asm/aarch64-types.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s" "/checkout/src/test/assembly/asm/aarch64-types.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/assembly/asm/aarch64-types.rs:100:11: error: CHECK: expected string not found in input
// CHECK: str {{.*}}x30
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s:35:13: note: scanning from here
issue_75761:
            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s:37:2: note: possible intended match here
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s:37:2: note: possible intended match here
 str x28, [sp, #-16]!

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-types/aarch64-types.s
Check file: /checkout/src/test/assembly/asm/aarch64-types.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
            30:  
            31:  .section .text.issue_75761,"ax",@progbits 
            32:  .globl issue_75761 
            33:  .p2align 2 
            34:  .type issue_75761,@function 
            35: issue_75761: 
check:100'0                 X error: no match found
            36:  .cfi_startproc 
check:100'0     ~~~~~~~~~~~~~~~~
            37:  str x28, [sp, #-16]! 
check:100'0     ~~~~~~~~~~~~~~~~~~~~~~
check:100'1      ?                     possible intended match
            38:  .cfi_def_cfa_offset 16 
check:100'0     ~~~~~~~~~~~~~~~~~~~~~~~~
            39:  .cfi_offset w28, -16 
check:100'0     ~~~~~~~~~~~~~~~~~~~~~~
            40:  //APP 
check:100'0     ~~~~~~~
            41:  //NO_APP 
check:100'0     ~~~~~~~~~~
            42:  ldr x28, [sp], #16 
check:100'0     ~~~~~~~~~~~~~~~~~~~~
             .
             .
>>>>>>
------------------------------------------
