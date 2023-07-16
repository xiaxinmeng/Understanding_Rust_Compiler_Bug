
Dump of assembler code for function test::p:
test.rs:
5	fn p() {
   0x000055555555ce30 <+0>:	push   %rbp
   0x000055555555ce31 <+1>:	mov    %rsp,%rbp
   0x000055555555ce34 <+4>:	sub    $0x30,%rsp
   0x000055555555ce38 <+8>:	lea    -0x30(%rbp),%rdi
   0x000055555555ce3c <+12>:	lea    0x54b7d(%rip),%rcx        # 0x5555555b19c0 <str.1>
   0x000055555555ce43 <+19>:	xor    %eax,%eax
   0x000055555555ce45 <+21>:	mov    %eax,%r8d

6	    println!("Hi");
=> 0x000055555555ce48 <+24>:	mov    0x270441(%rip),%rsi        # 0x5555557cd290 <_ZN4test1p15__STATIC_FMTSTR17hb6e039e832f7c61fE>
   0x000055555555ce4f <+31>:	mov    0x270442(%rip),%rdx        # 0x5555557cd298 <_ZN4test1p15__STATIC_FMTSTR17hb6e039e832f7c61fE+8>
   0x000055555555ce56 <+38>:	callq  0x55555555cdb0 <core::fmt::{{impl}}::new_v1>
   0x000055555555ce5b <+43>:	lea    -0x30(%rbp),%rdi
   0x000055555555ce5f <+47>:	callq  0x55555556b070 <std::io::stdio::_print>

7	}
   0x000055555555ce64 <+52>:	add    $0x30,%rsp
   0x000055555555ce68 <+56>:	pop    %rbp
   0x000055555555ce69 <+57>:	retq
End of assembler dump.
