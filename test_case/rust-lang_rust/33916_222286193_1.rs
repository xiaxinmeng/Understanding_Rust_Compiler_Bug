 asm
push   %rax
movl   $0xccddeeff,0x4(%rsp)
movl   $0x0,(%rsp)
movl   $0xccddeeff,(%rsp)
cmpl   $0xccddeeff,0x4(%rsp)
jne    5243 <_ZN4read4main17h1a4d109b8431532bE+0x23>
pop    %rax
retq   
