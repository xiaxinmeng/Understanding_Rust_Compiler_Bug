
gdb) b main
Breakpoint 1 at 0x40e0
(gdb) disas 0x40e0
Dump of assembler code for function main:
   0x000040e0 <+0>:     sub    $0x18,%esp
   0x000040e3 <+3>:     mov    0x26c88(%rip),%al        # 0x2ad71 <__rustc_debug_gdb_scripts_section__>
   0x000040e9 <+9>:     lea    -0x9f(%rip),%ecx        # 0x4050 <main::main>
   0x000040ef <+15>:    mov    %edi,0x14(%esp)
   0x000040f4 <+20>:    mov    %ecx,%edi
   0x000040f6 <+22>:    mov    0x14(%esp),%ecx
   0x000040fb <+27>:    mov    %esi,0x10(%esp)
   0x00004100 <+32>:    mov    %ecx,%esi
   0x00004102 <+34>:    mov    0x10(%esp),%edx
   0x00004107 <+39>:    mov    %al,0xf(%esp)
   0x0000410c <+44>:    callq  0x4190 <std::rt::lang_start>
   0x00004111 <+49>:    add    $0x18,%esp
   0x00004114 <+52>:    retq   
End of assembler dump.
(gdb) disas 0x4050
Dump of assembler code for function main::main:
   0x00004050 <+0>:     sub    $0x38,%esp
   0x00004053 <+3>:     mov    0x2ef5b(%rip),%eax        # 0x32fb4
   0x00004059 <+9>:     mov    0x2ef61(%rip),%ecx        # 0x32fc0
   0x0000405f <+15>:    mov    %ecx,0x30(%esp)
   0x00004064 <+20>:    mov    0x30(%esp),%ecx
   0x00004069 <+25>:    mov    %ecx,0x34(%esp)
   0x0000406e <+30>:    mov    %ecx,%edi
   0x00004070 <+32>:    lea    -0x116(%rip),%esi        # 0x3f60 <<&T as core::fmt::Display>::fmt>
   0x00004076 <+38>:    mov    %eax,0xc(%esp)
   0x0000407b <+43>:    callq  0x3fa0 <core::fmt::ArgumentV1::new>
   0x00004080 <+48>:    mov    %eax,0x8(%esp)
   0x00004085 <+53>:    mov    %edx,0x4(%esp)
   0x0000408a <+58>:    mov    0x8(%esp),%eax
   0x0000408f <+63>:    mov    %eax,0x28(%esp)
   0x00004094 <+68>:    mov    0x4(%esp),%ecx
   0x00004099 <+73>:    mov    %ecx,0x2c(%esp)
   0x0000409e <+78>:    lea    0x28(%rsp),%edx
   0x000040a2 <+82>:    lea    0x10(%rsp),%edi
   0x000040a6 <+86>:    mov    0xc(%esp),%esi
   0x000040ab <+91>:    mov    $0x2,%r8d
   0x000040b1 <+97>:    mov    %edx,(%esp)
   0x000040b5 <+101>:   mov    %r8d,%edx
   0x000040b8 <+104>:   mov    (%esp),%ecx
   0x000040bc <+108>:   mov    $0x1,%r8d
   0x000040c2 <+114>:   callq  0x4000 <core::fmt::Arguments::new_v1>
   0x000040c7 <+119>:   lea    0x10(%rsp),%edi
   0x000040cb <+123>:   callq  0x7b10 <std::io::stdio::_print>
   0x000040d0 <+128>:   add    $0x38,%esp
   0x000040d3 <+131>:   retq   
End of assembler dump.
(gdb) 
