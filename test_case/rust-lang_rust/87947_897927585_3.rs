
   0x0000555555a02eb6 <+134>:   mov    0x2a0(%r15),%rcx
   0x0000555555a02ebd <+141>:   lea    0x198(%rsp),%rdi
   0x0000555555a02ec5 <+149>:   lea    0x328(%rsp),%rsi
   0x0000555555a02ecd <+157>:   call   0x555555a42790 
   0x0000555555a02ed2 <+162>:   mov    0x198(%rsp),%rax
   0x0000555555a02eda <+170>:   movups 0x1a0(%rsp),%xmm0
   0x0000555555a02ee2 <+178>:   movaps %xmm0,0x10(%rsp)
   0x0000555555a02ee7 <+183>:   mov    0x1b0(%rsp),%rcx
   0x0000555555a02eef <+191>:   mov    %rcx,0x20(%rsp)
   0x0000555555a02ef4 <+196>:   cmp    $0x2,%rax
   0x0000555555a02ef8 <+200>:   jne    0x555555a0300f <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h749e4f0eba59cec2E+479>
