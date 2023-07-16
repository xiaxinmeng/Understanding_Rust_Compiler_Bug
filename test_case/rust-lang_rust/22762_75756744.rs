
   0x00000000001e901e <+3070>:  callq  0x1eaff0 <_ZN4sync4mpsc6shared15Packet$LT$T$GT$8try_recv20h9233234408304405675E>
   0x00000000001e9023 <+3075>:  movzwl 0xde0(%rsp),%eax
   0x00000000001e902b <+3083>:  cmp    $0xff,%eax
=> 0x00000000001e9030 <+3088>:  ja     0x1e9392 <_ZN4sync4mpsc17Receiver$LT$T$GT$4recv21h14764315989133460255E+3954>
   0x00000000001e9036 <+3094>:  movzbl %al,%eax
   0x00000000001e9039 <+3097>:  cmp    $0x1,%eax
   0x00000000001e903c <+3100>:  jne    0x1e9392 <_ZN4sync4mpsc17Receiver$LT$T$GT$4recv21h14764315989133460255E+3954>
   0x00000000001e9042 <+3106>:  lea    0xde0(%rsp),%r12
   0x00000000001e904a <+3114>:  mov    %r12,%rdi
   0x00000000001e904d <+3117>:  callq  0x228790 <_ZN4sync4mpsc8blocking6tokens20hc880a4e85e4c780bbnyE>
