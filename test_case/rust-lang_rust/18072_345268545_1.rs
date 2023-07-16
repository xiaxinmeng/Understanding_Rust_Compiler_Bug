asm
        pushl   %ebp
        pushl   %ebx
        pushl   %edi
        pushl   %esi
        movl    $12884901884, %eax
        calll   __rust_probestack
        subl    %eax, %esp
