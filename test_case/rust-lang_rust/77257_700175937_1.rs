asm
         5a1:   mov    0x40(%rsp),%edx
 20.80          mov    0x43(%rsp),%esi
  0.03          mov    %esi,0x183(%rsp)
                mov    %edx,0x180(%rsp)
                mov    $0x1,%dl 
                cmp    $0x3,%cl 
              ↓ jne    5d9      
                mov    0x180(%rsp),%ecx
  0.01          mov    0x183(%rsp),%edx
 23.64          mov    %edx,0x23(%rsp)
  0.12          mov    %ecx,0x20(%rsp)
                cmp    $0x2,%al 
                sete   %dl      
  0.07   5d9:   mov    0x20(%rsp),%eax
  0.01          mov    0x23(%rsp),%ecx
 18.20          mov    %ecx,0xb3(%rsp)
  0.01          mov    %eax,0xb0(%rsp)
                test   %dl,%dl  
  0.00        ↓ jne    1546     
                mov    %r14,0x18(%rsp)
  0.00          mov    0x218(%rsp),%rax
  0.00          mov    (%rax),%rdi
  0.01          mov    0x30(%rbx),%rax
  0.01          mov    0x8(%rbx),%r14
  0.14          mov    0x10(%rbx),%rsi
                mov    0x20(%rbx),%rdx
                mov    0x18(%rbx),%r10
                movzbl (%rax),%ecx
                cmp    $0x2,%rcx
              ↓ je     629      
