
   0xffff800100f35668 <+3060>:  and  %i1, 3, %i0        ! begin inlined lint_if_path_starts_with_module
   0xffff800100f3566c <+3064>:  cmp  %i0, 1                        ! CrateLint::SimplePath
   0xffff800100f35670 <+3068>:  be  L1                                ! jumps to the crash
   0xffff800100f35674 <+3072>:  st  %i4, [ %fp + 0x69f ]
   0xffff800100f35678 <+3076>:  cmp  %i0, 2                        ! CrateLint::UsePath (case 1)
   0xffff800100f3567c <+3080>:  be  0xffff800100f35690 <MYSELF+3100>
   0xffff800100f35680 <+3084>:  ldx  [ %fp + 0x63f ], %g2
   0xffff800100f35684 <+3088>:  cmp  %i0, 3                        ! CrateLint::QPathTrait
   0xffff800100f35688 <+3092>:  bne  0xffff800100f359b0 <MYSELF+3900> ! jump if "No" to end of inlined fn
   0xffff800100f3568c <+3096>:  nop
   0xffff800100f35690 <+3100>:  ld  [ %fp + 0x71f ], %i0
   0xffff800100f35694 <+3104>:  stx  %i0, [ %fp + 0x65f ]
   0xffff800100f35698 <+3108>:  L2                                      ! jumps accross the crash (cases 2 & 3)
   0xffff800100f3569c <+3112>:  add  %fp, 0x69f, %i0
