
 44.00 │40:   test %rcx,%rcx                                                                                                                               
       │    ↓ je   5d                                                                                                                                      
 16.00 │      mov  %rax,%rdx                                                                                                                               
 12.00 │      add  $0x10,%rax                                                                                                                              
  8.00 │      add  $0xfffffffffffffff0,%rcx                                                                                                                
 20.00 │      cmp  %rbx,(%rdx)
       │    ↑ jne  40
