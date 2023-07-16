
   0x00007fe22d9b9a34 <+68>:	call   0x7fe22d9b6ee0 <_ZN4core4char7methods15encode_utf8_raw17hc73da9e6e7004300E>
   ; returned slice pointer is in %rax, returned slice length is in %rdx, both correct
   0x00007fe22d9b9a39 <+73>:	mov    %rdx,0x18(%rsp)
   0x00007fe22d9b9a3e <+78>:	mov    %rax,0x38(%rsp)
   0x00007fe22d9b9a43 <+83>:	mov    %rdx,0x40(%rsp)
   0x00007fe22d9b9a48 <+88>:	mov    0x18(%rsp),%rdx
   0x00007fe22d9b9a4d <+93>:	mov    (%rsp),%rsi
   0x00007fe22d9b9a51 <+97>:	mov    0x8(%rsp),%rdi
   ; input function pointer is in %rsi, length in %rdi
   0x00007fe22d9b9a56 <+102>:	call   0x7fe22d9b8380 <_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17he0dae454987f3de8E>
