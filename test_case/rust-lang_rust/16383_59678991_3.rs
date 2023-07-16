
$ clang pushPop.c -S -o -O
...
.Ltmp5:
        .cfi_def_cfa_register %rbp
        subq    $16, %rsp
        movq    $42, -8(%rbp)
        movl    $42, %esi
        #APP
        push %rsi; pop %rsi
        #NO_APP
        movq    %rsi, -8(%rbp)
        movl    $.L.str, %edi
        xorb    %al, %al
        callq   printf
        xorl    %eax, %eax
        addq    $16, %rsp
        popq    %rbp
        ret
...
$ clang --version
Ubuntu clang version 3.0-6ubuntu3 (tags/RELEASE_30/final) (based on LLVM 3.0)
Target: x86_64-pc-linux-gnu
Thread model: posix
