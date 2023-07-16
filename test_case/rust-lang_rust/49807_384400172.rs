
(gdb) bt                                      
#0  <(f64, isize, isize, f64) as by_value_self_argument_in_trait_impl::Trait>::method (self=...)
    at /home/ubuntu/src/rustc/bionic-arm64/rustc-1.25.0+dfsg1+llvm/src/test/debuginfo/by-value-self-argument-in-trait-impl.rs:82
#1  0x0000aaaaaaaab320 in by_value_self_argument_in_trait_impl::main () at /home/ubuntu/src/rustc/bionic-arm64/rustc-1.25.0+dfsg1+llvm/src/test/debuginfo/by-value-self-argument-in-trait-impl.rs:90
(gdb) p self                                                                                                                                                                                             
$5 = (1.3906711615485821e-309, 3333, 281474976707216, 1.3906711615500445e-309)
(gdb) info registers                                
x0             0xfffffffff170   281474976706928     
x1             0xd05    3333                                                                             
x2             0xfffffffff290   281474976707216     
x3             0xfffffffff298   281474976707224     
x4             0xffffb7fd1258   281473768559192     
x5             0x0      0                           
x6             0x1      1                           
x7             0x0      0                                                                                                                                                                                          
x8             0xfffffffff150   281474976706896     
x9             0x1a0a   6666                        
x10            0x15b3   5555                        
x11            0x40b15c8000000000       4661608794130743296
x12            0x1      1
x13            0x270f   9999
x14            0x2      2
x15            0x0      0
x16            0xffffb7fd0ac8   281473768557256
x17            0xffffb7f5acbc   281473768074428
x18            0xffffb7ea7a70   281473767340656
x19            0xfffffffff250   281474976707152
x20            0xfffffffff290   281474976707216
x21            0xffffb7fd1258   281473768559192
x22            0x0      0
x23            0x0      0
x24            0x0      0
x25            0x0      0
x26            0x0      0
x27            0x0      0
x28            0x0      0
x29            0xfffffffff130   281474976706864
x30            0xaaaaaaaab320   187649984475936
sp             0xfffffffff0f0   0xfffffffff0f0
pc             0xaaaaaaaab268   0xaaaaaaaab268 <<(f64, isize, isize, f64) as by_value_self_argument_in_trait_impl::Trait>::method+12>
cpsr           0x60000000       [ EL=0 C Z ]
fpsr           0x0      0
fpcr           0x0      0
