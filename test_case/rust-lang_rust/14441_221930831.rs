
$ gdb ./bin/rustc
[...]
(gdb) r --version
[...]
Program received signal SIGILL, Illegal instruction.
0xb7b4c94d in str::str.ToOwned::to_owned::h9fc80730fafb5634FVe ()
   from /extra1/rustc32/bin/../lib/libstd-db5a760f.so
(gdb) bt
#0  0xb7b4c94d in str::str.ToOwned::to_owned::h9fc80730fafb5634FVe ()
   from /extra1/rustc32/bin/../lib/libstd-db5a760f.so
#1  0xb7b42ecf in rt::lang_start::hdd382d1070c1d350rKz ()
   from /extra1/rustc32/bin/../lib/libstd-db5a760f.so
#2  0xb7fff78e in main ()
(gdb) x/i $eip
=> 0xb7b4c94d <_ZN3str11str.ToOwned8to_owned20h9fc80730fafb5634FVeE+141>:      movsd  0x20(%esp),%xmm0
(gdb) disassemble
Dump of assembler code for function _ZN3str11str.ToOwned8to_owned20h9fc80730fafb5634FVeE:
   0xb7b4c8c0 <+0>:     push   %ebp
   0xb7b4c8c1 <+1>:     push   %ebx
   0xb7b4c8c2 <+2>:     push   %edi
   0xb7b4c8c3 <+3>:     push   %esi
   0xb7b4c8c4 <+4>:     sub    $0x2c,%esp
   0xb7b4c8c7 <+7>:     call   0xb7b4c8cc <_ZN3str11str.ToOwned8to_owned20h9fc80730fafb5634FVeE+12>
   0xb7b4c8cc <+12>:    pop    %ebx
   0xb7b4c8cd <+13>:    add    $0xd9734,%ebx
   0xb7b4c8d3 <+19>:    mov    0x48(%esp),%esi
   0xb7b4c8d7 <+23>:    test   %esi,%esi
   0xb7b4c8d9 <+25>:    js     0xb7b4c976 <_ZN3str11str.ToOwned8to_owned20h9fc80730fafb5634FVeE+182>
   0xb7b4c8df <+31>:    mov    $0x1,%eax
   0xb7b4c8e4 <+36>:    je     0xb7b4c8fe <_ZN3str11str.ToOwned8to_owned20h9fc80730fafb5634FVeE+62>
   0xb7b4c8e6 <+38>:    mov    %esi,(%esp)
   0xb7b4c8e9 <+41>:    movl   $0x1,0x4(%esp)
   0xb7b4c8f1 <+49>:    call   0xb7afd110 <__rust_allocate@plt>
   0xb7b4c8f6 <+54>:    test   %eax,%eax
   0xb7b4c8f8 <+56>:    je     0xb7b4c9b5 <_ZN3str11str.ToOwned8to_owned20h9fc80730fafb5634FVeE+245>
   0xb7b4c8fe <+62>:    mov    %eax,0x20(%esp)
   0xb7b4c902 <+66>:    mov    %esi,0x24(%esp)
   0xb7b4c906 <+70>:    movl   $0x0,0x28(%esp)
   0xb7b4c90e <+78>:    lea    0x20(%esp),%ecx
   0xb7b4c912 <+82>:    mov    %esi,%edx
   0xb7b4c914 <+84>:    call   0xb7b4c9e0 <_ZN3vec12Vec$LT$T$GT$7reserve20h8526356496296567822E>
   0xb7b4c919 <+89>:    mov    0x40(%esp),%edi
   0xb7b4c91d <+93>:    test   %esi,%esi
   0xb7b4c91f <+95>:    je     0xb7b4c945 <_ZN3str11str.ToOwned8to_owned20h9fc80730fafb5634FVeE+133>
   0xb7b4c921 <+97>:    mov    0x44(%esp),%eax
   0xb7b4c925 <+101>:   mov    0x28(%esp),%ebp
   0xb7b4c929 <+105>:   mov    0x20(%esp),%ecx
   0xb7b4c92d <+109>:   add    %ebp,%ecx
   0xb7b4c92f <+111>:   mov    %esi,0x8(%esp)
   0xb7b4c933 <+115>:   mov    %eax,0x4(%esp)
   0xb7b4c937 <+119>:   mov    %ecx,(%esp)
   0xb7b4c93a <+122>:   call   0xb7afd480 <memcpy@plt>
   0xb7b4c93f <+127>:   add    %esi,%ebp
   0xb7b4c941 <+129>:   mov    %ebp,0x28(%esp)
   0xb7b4c945 <+133>:   mov    0x28(%esp),%eax
   0xb7b4c949 <+137>:   mov    %eax,0x18(%esp)
=> 0xb7b4c94d <+141>:   movsd  0x20(%esp),%xmm0
   0xb7b4c953 <+147>:   movsd  %xmm0,0x10(%esp)
   0xb7b4c959 <+153>:   mov    0x18(%esp),%eax
   0xb7b4c95d <+157>:   mov    %eax,0x8(%edi)
   0xb7b4c960 <+160>:   movsd  0x10(%esp),%xmm0
   0xb7b4c966 <+166>:   movsd  %xmm0,(%edi)
   0xb7b4c96a <+170>:   mov    %edi,%eax
   0xb7b4c96c <+172>:   add    $0x2c,%esp
   0xb7b4c96f <+175>:   pop    %esi
   0xb7b4c970 <+176>:   pop    %edi
   0xb7b4c971 <+177>:   pop    %ebx
   0xb7b4c972 <+178>:   pop    %ebp
---Type <return> to continue, or q <return> to quit---
   0xb7b4c973 <+179>:   ret    $0x4
   0xb7b4c976 <+182>:   mov    -0xe0(%ebx),%eax
   0xb7b4c97c <+188>:   mov    %eax,(%esp)
   0xb7b4c97f <+191>:   call   0xb7afdbf0 <_ZN9panicking5panic20hcdb434c1cdedc531SaME@plt>
   0xb7b4c984 <+196>:   mov    %eax,%esi
   0xb7b4c986 <+198>:   mov    0x24(%esp),%eax
   0xb7b4c98a <+202>:   test   %eax,%eax
   0xb7b4c98c <+204>:   je     0xb7b4c9ad <_ZN3str11str.ToOwned8to_owned20h9fc80730fafb5634FVeE+237>
   0xb7b4c98e <+206>:   cmp    $0x1d1d1d1d,%eax
   0xb7b4c993 <+211>:   je     0xb7b4c9ad <_ZN3str11str.ToOwned8to_owned20h9fc80730fafb5634FVeE+237>
   0xb7b4c995 <+213>:   mov    0x20(%esp),%ecx
   0xb7b4c999 <+217>:   mov    %eax,0x4(%esp)
   0xb7b4c99d <+221>:   mov    %ecx,(%esp)
   0xb7b4c9a0 <+224>:   movl   $0x1,0x8(%esp)
   0xb7b4c9a8 <+232>:   call   0xb7afcec0 <__rust_deallocate@plt>
   0xb7b4c9ad <+237>:   mov    %esi,(%esp)
   0xb7b4c9b0 <+240>:   call   0xb7afe1d0 <_Unwind_Resume@plt>
   0xb7b4c9b5 <+245>:   call   0xb7afda10 <_ZN3oom3oom20h3d2cfcfffd56b89aPzbE@plt>
End of assembler dump.
