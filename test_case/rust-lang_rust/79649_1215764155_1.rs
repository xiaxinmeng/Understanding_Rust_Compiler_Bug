
wink@3900x 22-08-15T19:52:32.960Z:~/prgs/rust/myrepos/exper-code-coverage (main)
$ cargo asm --build-type debug --asm-style=att exper_code_coverage::short_circuit_and::short_circuit_and
exper_code_coverage::short_circuit_and::short_circuit_and (src/short_circuit_and.rs:1):
 subq    $28, %rsp
 movl    %edx, (%rsp)
 movl    %ecx, 4(%rsp)
 movl    %edi, 12(%rsp)
 movl    %esi, 16(%rsp)
 movl    %edx, 20(%rsp)
 movl    %ecx, 24(%rsp)
 cmpl    %esi, %edi
 jl      .LBB1_2
 movb    $0, 11(%rsp)
 jmp     .LBB1_3
.LBB1_2:
 movl    (%rsp), %eax
 movl    4(%rsp), %ecx
 cmpl    %ecx, %eax
 setl    %al
 andb    $1, %al
 movb    %al, 11(%rsp)
.LBB1_3:
 movb    11(%rsp), %al
 andb    $1, %al
 movzbl  %al, %eax
 addq    $28, %rsp
 retq
