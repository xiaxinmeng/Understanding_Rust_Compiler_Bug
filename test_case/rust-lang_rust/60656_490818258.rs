
$ cargo asm foo::serialize
foo::serialize:
 sub     rsp, 16
 mov     rax, rdi
 mov     qword, ptr, [rsp], rsi
 mov     qword, ptr, [rsp, +, 8], rdx
 mov     rcx, qword, ptr, [rsp]
 mov     qword, ptr, [rdi], rcx
 mov     rcx, qword, ptr, [rsp, +, 8]
 mov     qword, ptr, [rdi, +, 8], rcx
 add     rsp, 16
 ret
