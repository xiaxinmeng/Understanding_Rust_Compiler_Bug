asm
GetMyStruct:                  ; sparc32-linux-gnu
	ld	[%sp+64], %g1 ; Load return value pointer
	mov	1, %g2        ; Put true in register
	mov	%g1, %o0      ; Put pointer in %o0 for caller
	jmp	%o7+12        ; Return (skipping over unimp)
	 stb	%g2, [%g1]    ; Write true to struct (delay slot)
