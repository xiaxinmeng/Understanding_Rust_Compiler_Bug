
Dump of assembler code for function _ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E:
   0x0000000000000000 <+0>:	48 83 ec 28	sub    $0x28,%rsp
   0x0000000000000004 <+4>:	48 89 3c 24	mov    %rdi,(%rsp)
   0x0000000000000008 <+8>:	48 89 74 24 08	mov    %rsi,0x8(%rsp)
   0x000000000000000d <+13>:	48 8b 04 24	mov    (%rsp),%rax
   0x0000000000000011 <+17>:	48 85 c0	test   %rax,%rax
   0x0000000000000014 <+20>:	74 12	je     0x28 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+40>
   0x0000000000000016 <+22>:	eb 00	jmp    0x18 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+24>
   0x0000000000000018 <+24>:	eb 20	jmp    0x3a <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+58>
   0x000000000000001a <+26>:	48 8b 7c 24 18	mov    0x18(%rsp),%rdi
   0x000000000000001f <+31>:	e8 00 00 00 00	callq  0x24 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+36>
   0x0000000000000024 <+36>:	0f 0b	ud2    
   0x0000000000000026 <+38>:	0f 0b	ud2    
   0x0000000000000028 <+40>:	48 83 3c 24 00	cmpq   $0x0,(%rsp)
   0x000000000000002d <+45>:	74 3c	je     0x6b <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+107>
   0x000000000000002f <+47>:	eb 3f	jmp    0x70 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+112>
   0x0000000000000031 <+49>:	48 83 3c 24 00	cmpq   $0x0,(%rsp)
   0x0000000000000036 <+54>:	74 31	je     0x69 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+105>
   0x0000000000000038 <+56>:	eb e0	jmp    0x1a <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+26>
   0x000000000000003a <+58>:	48 8b 44 24 08	mov    0x8(%rsp),%rax
   0x000000000000003f <+63>:	48 89 44 24 10	mov    %rax,0x10(%rsp)
   0x0000000000000044 <+68>:	48 8d 3d 00 00 00 00	lea    0x0(%rip),%rdi        # 0x4b <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+75>
   0x000000000000004b <+75>:	48 8d 0d 00 00 00 00	lea    0x0(%rip),%rcx        # 0x52 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+82>
   0x0000000000000052 <+82>:	48 8b 05 00 00 00 00	mov    0x0(%rip),%rax        # 0x59 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+89>
   0x0000000000000059 <+89>:	be 2b 00 00 00	mov    $0x2b,%esi
   0x000000000000005e <+94>:	48 8d 54 24 10	lea    0x10(%rsp),%rdx
   0x0000000000000063 <+99>:	ff d0	callq  *%rax
   0x0000000000000065 <+101>:	eb 0b	jmp    0x72 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+114>
   0x0000000000000067 <+103>:	eb c8	jmp    0x31 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+49>
   0x0000000000000069 <+105>:	eb af	jmp    0x1a <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+26>
   0x000000000000006b <+107>:	48 83 c4 28	add    $0x28,%rsp
   0x000000000000006f <+111>:	c3	retq   
   0x0000000000000070 <+112>:	eb f9	jmp    0x6b <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+107>
   0x0000000000000072 <+114>:	0f 0b	ud2    
   0x0000000000000074 <+116>:	48 89 44 24 18	mov    %rax,0x18(%rsp)
   0x0000000000000079 <+121>:	89 54 24 20	mov    %edx,0x20(%rsp)
   0x000000000000007d <+125>:	eb e8	jmp    0x67 <_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hc6a8d8550c5348d4E+103>
