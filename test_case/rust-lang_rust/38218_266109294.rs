
(gdb) x/10i $pc
=> 0x555555559946 <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+38>:	vmovdqu8 (%rdi),%xmm0
   0x55555555994c <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+44>:	vmovdqu8 %xmm0,(%rsp)
   0x555555559953 <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+51>:	
    lea    0x255026(%rip),%rax        # 0x5555557ae980 <_ZN4core6result13unwrap_failed15__STATIC_FMTSTR17h9faf142ae795601eE>
   0x55555555995a <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+58>:	vmovdqu64 (%rax),%xmm0
   0x555555559960 <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+64>:	lea    0x20(%rsp),%rax
   0x555555559965 <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+69>:	mov    %rax,0x30(%rsp)
   0x55555555996a <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+74>:	
    lea    0x7f(%rip),%rax        # 0x5555555599f0 <_ZN55_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h178215f4e32f16d5E>
   0x555555559971 <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+81>:	mov    %rax,0x38(%rsp)
   0x555555559976 <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+86>:	lea    (%rsp),%rax
   0x55555555997a <_ZN4core6result13unwrap_failed17h98e39d97d47cde50E+90>:	mov    %rax,0x40(%rsp)
