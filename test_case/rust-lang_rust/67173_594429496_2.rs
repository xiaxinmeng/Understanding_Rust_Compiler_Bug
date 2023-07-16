
0x4002ab <DORUN+235>    callq  0x41c650 <fpc_get_output>
0x4002b0 <DORUN+240>    mov    %rax,%rbx
0x4002b3 <DORUN+243>    lea    0x83c76(%rip),%rdx        # 0x483f30 
0x4002ba <DORUN+250>    mov    %rbx,%rsi                                         
0x4002bd <DORUN+253>    mov    $0x0,%edi                                       
0x4002c2 <DORUN+258>    callq  0x41c8f0 <fpc_write_text_shortstr>  
0x4002c7 <DORUN+263>    callq  0x4169d0 <fpc_iocheck>                  
0x4002cc <DORUN+268>    mov    %rbx,%rdi                                        
0x4002cf <DORUN+271>    callq  0x41c820 <fpc_writeln_end>             
0x4002d4 <DORUN+276>    callq  0x4169d0 <fpc_iocheck>                  
0x4002d9 <DORUN+281>    mov    -0x8(%rbp),%rdi                              
0x4002dd <DORUN+285>    mov    -0x8(%rbp),%rax                             
0x4002e1 <DORUN+289>    mov    (%rax),%rax                                    
0x4002e4 <DORUN+292>    callq  *0x1f8(%rax)                                    
0x4002ea <DORUN+298>    callq  0x413c40 <fpc_popaddrstack>        
0x4002ef <DORUN+303>    lea    -0x10(%rbp),%rdi                              
0x4002f3 <DORUN+307>    callq  0x40ac60 <fpc_ansistr_decr_ref>    
0x4002f8 <DORUN+312>    mov    -0x70(%rbp),%rax            
0x4002fc <DORUN+316>    test   %rax,%rax                     
0x4002ff <DORUN+319>    je     0x400306 <DORUN+326>
0x400301 <DORUN+321>    callq  0x413dd0 <fpc_reraise>
0x400306 <DORUN+326>    mov    -0x78(%rbp),%rbx        
0x40030a <DORUN+330>    leaveq                                     
0x40030b <DORUN+331>    retq                                         
