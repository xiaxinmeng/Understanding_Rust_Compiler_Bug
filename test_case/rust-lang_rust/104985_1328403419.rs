quote
     The MIPS3 extensions to the MIPS instruction set, introduced in the R4000
     processors, are primarily to support 64-bit addresses and arithmetic and
     a larger floating point register set. The 64-bit addressing features are
     only supported on machines	running	a 64-bit kernel.  The 64-bit
     arithmetic	features are supported on all machines running IRIX 6.2	and
     later releases (R4K and later CPUs).

     The MIPS3 instruction set extensions provide the following	features:

     o	  64-bit integer registers, with a complete set	of instructions	to
	  perform 64-bit integer arithmetic operations.

     o	  64-bit addresses and pointers.  The R4000 family and later MIPS
	  processors support a 64-bit flat address space.

     o	  Thirty two 64-bit floating point registers.  The R4000 family
	  supports two floating	point register (FPR) modes, determining	how
	  many 64-bit FPRs are available:  16-FPR mode and 32-FPR mode.	 The
	  16-FPR mode is compatible with the R2000/R3000 and is	available
	  using	the default (-mips1) or	-mips2 compiler	options.  The 32-FPR
	  mode is not compatible with the MIPS1	or MIPS2 model.	 Hence it is
	  only supported by the	-mips3 and -mips4 options.
