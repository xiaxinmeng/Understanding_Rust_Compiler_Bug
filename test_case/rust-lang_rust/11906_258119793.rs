
   0x00005555555599c0 <+0>: push   %rbp
   0x00005555555599c1 <+1>: mov    %rsp,%rbp
   0x00005555555599c4 <+4>: sub    $0x140,%rsp                 # breakpoint with `rbreak non_immediate` with frame pointer elimination enabled
   0x00005555555599cb <+11>:    mov    $0x40,%eax
   0x00005555555599d0 <+16>:    mov    %eax,%ecx
   0x00005555555599d2 <+18>:    lea    -0x80(%rbp),%rdx
   0x00005555555599d6 <+22>:    lea    -0x40(%rbp),%r8
   0x00005555555599da <+26>:    mov    %rdi,-0x110(%rbp)
   0x00005555555599e1 <+33>:    mov    %r8,%rdi
   0x00005555555599e4 <+36>:    mov    -0x110(%rbp),%r8
   0x00005555555599eb <+43>:    mov    %rsi,-0x118(%rbp)
   0x00005555555599f2 <+50>:    mov    %r8,%rsi
   0x00005555555599f5 <+53>:    mov    %rdx,-0x120(%rbp)
   0x00005555555599fc <+60>:    mov    %rcx,%rdx
   0x00005555555599ff <+63>:    mov    %rcx,-0x128(%rbp)
   0x0000555555559a06 <+70>:    callq  0x5555555595c0 <memcpy@plt> # argument a gets initialized
   0x0000555555559a0b <+75>:    mov    -0x118(%rbp),%rcx
   0x0000555555559a12 <+82>:    mov    -0x120(%rbp),%rdx
   0x0000555555559a19 <+89>:    mov    %rdx,%rdi
   0x0000555555559a1c <+92>:    mov    %rcx,%rsi
   0x0000555555559a1f <+95>:    mov    -0x128(%rbp),%rdx
   0x0000555555559a26 <+102>:   callq  0x5555555595c0 <memcpy@plt> # argument b gets initialized
   0x0000555555559a2b <+107>:   mov    $0x40,%eax
   0x0000555555559a30 <+112>:   mov    %eax,%ecx
   0x0000555555559a32 <+114>:   lea    -0x108(%rbp),%rdx
   0x0000555555559a39 <+121>:   lea    -0x80(%rbp),%rsi
   0x0000555555559a3d <+125>:   lea    -0xc8(%rbp),%rdi
   0x0000555555559a44 <+132>:   lea    -0x40(%rbp),%r8
   0x0000555555559a48 <+136>:   mov    %rsi,-0x130(%rbp)           # breakpoint with `rbreak non_immediate` with frame pointer elimination disabled
   0x0000555555559a4f <+143>:   mov    %r8,%rsi
   0x0000555555559a52 <+146>:   mov    %rdx,-0x138(%rbp)
   0x0000555555559a59 <+153>:   mov    %rcx,%rdx
   0x0000555555559a5c <+156>:   mov    %rcx,-0x140(%rbp)
   0x0000555555559a63 <+163>:   callq  0x5555555595c0 <memcpy@plt>
   0x0000555555559a68 <+168>:   mov    -0x130(%rbp),%rcx
   0x0000555555559a6f <+175>:   mov    -0x138(%rbp),%rdx
   0x0000555555559a76 <+182>:   mov    %rdx,%rdi
   0x0000555555559a79 <+185>:   mov    %rcx,%rsi
   0x0000555555559a7c <+188>:   mov    -0x140(%rbp),%rdx
   0x0000555555559a83 <+195>:   callq  0x5555555595c0 <memcpy@plt>
   0x0000555555559a88 <+200>:   callq  0x55555555a020 <function_arg_initialization::zzz>
   0x0000555555559a8d <+205>:   add    $0x140,%rsp
   0x0000555555559a94 <+212>:   pop    %rbp
   0x0000555555559a95 <+213>:   retq   
