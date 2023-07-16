
$ objdump -Cd target/release/deps/quick_tests-23d74409b25bdbab | less
...
0000000000004eb0 <quick_tests::main>:
    4eb0:       55                      push   %rbp
    4eb1:       41 57                   push   %r15
    4eb3:       41 56                   push   %r14
    4eb5:       41 55                   push   %r13
    4eb7:       41 54                   push   %r12
    4eb9:       53                      push   %rbx
    4eba:       48 81 ec 58 04 00 00    sub    $0x458,%rsp   <=== 1025 + 0x57
    4ec1:       48 8d 7c 24 57          lea    0x57(%rsp),%rdi
...
