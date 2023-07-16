
   │0xffff8 <printf_core>           push   {r4, r5, r6, r7, r8, r9, r10, r11, lr}
   │0xffffc <printf_core+4>         vpush  {d8-d14} 
  >│0x100000 <printf_core+8>        vmov.f64       d10, #112       ; 0x3f800000  1.0    
   │0x100004 <printf_core+12>       vmov.f64       d11, #120       ; 0x3fc00000  1.5      
   │0x100008 <printf_core+16>       vldr   d8, [pc, #904]  ; 0x100398 <printf_core+928>   
   │0x10000c <printf_core+20>       vldr   d9, [pc, #908]  ; 0x1003a0 <printf_core+936>
