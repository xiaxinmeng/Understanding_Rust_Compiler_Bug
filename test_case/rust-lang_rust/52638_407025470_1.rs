asm
GetMyStruct:                 ; sparc64-linux-gnu
	mov	1, %o0       ; Put true in register
	jmp	%o7+8        ; Return (no unimp on sparc64)
	 sllx	%o0, 56, %o0 ; Move true to top byte (delay slot)
