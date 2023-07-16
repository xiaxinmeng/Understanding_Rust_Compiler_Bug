
   ; SPFlagOptimized is 16, $g2 contains the previous value of spflags
   ; if self.sess().opts.optimize != config::OptLevel::No {
   ;           spflags |= DISPFlags::SPFlagOptimized;
   ; }
   or  %g2, 0x10, %i3
   cmp  %g3, 0
   move  %icc, %g2, %i3
   ; move to high word and pass as parameter
   sllx  %i3, 0x20, %i3
   stx  %l3, [ %sp + 0x8d7 ]
   ; ...
   call  0xfff800010c44d920 <LLVMRustDIBuilderCreateFunction@got.plt>
