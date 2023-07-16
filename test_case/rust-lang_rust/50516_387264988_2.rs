
 <_ZN3std2fs10create_dir17h0c9a5398928d3c8dE>:                               
d10183ff        sub     sp, sp, #0x60                                        
a9057bfd        stp     x29, x30, [sp, #80]                                  
910143fd        add     x29, sp, #0x50                                       
9100a3e9        add     x9, sp, #0x28                                        
f9000120        str     x0, [x9]                                             
f9001be1        str     x1, [sp, #48]                                        
f90013e8        str     x8, [sp, #32]                                        
94000fcc        bl      79f4 <_ZN3std2fs10DirBuilder3new17h4d87461ec68baf84E>
2a0003e9        mov     w9, w0                                               
b3407d28        bfxil   x8, x9, #0, #32                                      
2a0103e9        mov     w9, w1                                               
b3600128        bfi     x8, x9, #32, #1                                      
f9000fe8        str     x8, [sp, #24]                                        
