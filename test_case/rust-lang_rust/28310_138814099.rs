 nasm
main:
    push    rbp
    mov rbp, rsp
    sub rsp, 16

    lea rdi, [rbp - 8]
    mov byte ptr [rbp - 1], 61
    mov byte ptr [rbp - 8], -44 ; this is where the drop flag is set
    call    _ZN7Dropper9drop.256317hcf85f43760592a92E

    add rsp, 16
    pop rbp
    ret
