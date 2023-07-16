
(gdb) b main
Breakpoint 1 at 0x40a0
(gdb) disas 0x40a0
Dump of assembler code for function main:
   0x000040a0 <+0>:     sub    $0x18,%esp
   0x000040a3 <+3>:     mov    0x26ccc(%rip),%al        # 0x2ad75 <__rustc_debug_gdb_scripts_section__>
   0x000040a9 <+9>:     lea    -0x9f(%rip),%ecx        # 0x4010 <main::main>
   0x000040af <+15>:    mov    %edi,0x14(%esp)
   0x000040b4 <+20>:    mov    %ecx,%edi
   0x000040b6 <+22>:    mov    0x14(%esp),%ecx
   0x000040bb <+27>:    mov    %esi,0x10(%esp)
   0x000040c0 <+32>:    mov    %ecx,%esi
   0x000040c2 <+34>:    mov    0x10(%esp),%edx
   0x000040c7 <+39>:    mov    %al,0xf(%esp)
   0x000040cc <+44>:    callq  0x4150 <std::rt::lang_start>
   0x000040d1 <+49>:    add    $0x18,%esp
   0x000040d4 <+52>:    retq   
End of assembler dump.
(gdb) disas 0x4010
Dump of assembler code for function main::main:
   0x00004010 <+0>:     sub    $0x38,%esp
   0x00004013 <+3>:     mov    0x2ffd2,%esi
   0x0000401a <+10>:    mov    0x2ef94(%rip),%eax        # 0x32fb4
   0x00004020 <+16>:    mov    0x2ef92(%rip),%ecx        # 0x32fb8
   0x00004026 <+22>:    mov    %ecx,0x30(%esp)
   0x0000402b <+27>:    mov    0x30(%esp),%ecx
   0x00004030 <+32>:    mov    %ecx,0x34(%esp)
   0x00004035 <+37>:    mov    %ecx,%edi
   0x00004037 <+39>:    mov    %eax,0xc(%esp)
   0x0000403c <+44>:    callq  0x3f60 <core::fmt::ArgumentV1::new>
   0x00004041 <+49>:    mov    %eax,0x8(%esp)
   0x00004046 <+54>:    mov    %edx,0x4(%esp)
   0x0000404b <+59>:    mov    0x8(%esp),%eax
   0x00004050 <+64>:    mov    %eax,0x28(%esp)
   0x00004055 <+69>:    mov    0x4(%esp),%ecx
   0x0000405a <+74>:    mov    %ecx,0x2c(%esp)
   0x0000405f <+79>:    lea    0x28(%rsp),%edx
   0x00004063 <+83>:    lea    0x10(%rsp),%edi
   0x00004067 <+87>:    mov    0xc(%esp),%esi
   0x0000406c <+92>:    mov    $0x2,%r8d
   0x00004072 <+98>:    mov    %edx,(%esp)
   0x00004076 <+102>:   mov    %r8d,%edx
   0x00004079 <+105>:   mov    (%esp),%ecx
   0x0000407d <+109>:   mov    $0x1,%r8d
   0x00004083 <+115>:   callq  0x3fc0 <core::fmt::Arguments::new_v1>
   0x00004088 <+120>:   lea    0x10(%rsp),%edi
   0x0000408c <+124>:   callq  0x7ad0 <std::io::stdio::_print>
   0x00004091 <+129>:   add    $0x38,%esp
   0x00004094 <+132>:   retq   
End of assembler dump.
(gdb) 
