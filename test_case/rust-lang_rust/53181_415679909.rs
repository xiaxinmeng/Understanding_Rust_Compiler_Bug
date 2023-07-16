
L1:
   0xffff800100f3583c <+3528>:  ldx  [ %fp + 0x627 ], %i0
   0xffff800100f35840 <+3532>:  or  %i0, 3, %i0
   0xffff800100f35844 <+3536>:  ldx  [ %fp + 0x63f ], %g2
L2:
   0xffff800100f35848 <+3540>:  ld  [ %i2 ], %i1
   0xffff800100f3584c <+3544>:  cmp  %i1, 0xc                        ! keywords::Extern.name()
   0xffff800100f35850 <+3548>:  be  0xffff800100f35864 <MYSELF+3568>
=> 0xffff800100f35854 <+3552>:  ld  [ %i0 ], %i0
   0xffff800100f35858 <+3556>:  cmp  %i1, 1                          ! keywords::CreateRoot.name()
   0xffff800100f3585c <+3560>:  bne  0xffff800100f359b0 <MYSELF+3900> ! return from inlined lint_if_path_starts_with_module
