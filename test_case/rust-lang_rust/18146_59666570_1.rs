 asm
main:
       0:   48 83 ec 18                                     subq    $24, %rsp
       4:   48 89 7c 24 10                                  movq    %rdi, 16(%rsp)
       9:   48 8b 7c 24 10                                  movq    16(%rsp), %rdi
...
