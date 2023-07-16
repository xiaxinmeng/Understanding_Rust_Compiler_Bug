asm
_ZN4core4sync6atomic10atomic_add17h3d7ac1a1809749eaE: // @_ZN4core4sync6atomic10atomic_add17h3d7ac1a1809749eaE
	.cfi_startproc
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	adrp	x9, .LJTI0_0
	and	x8, x2, #0xff
	add	x9, x9, :lo12:.LJTI0_0
	strb	w2, [sp, #4]
	adr	x10, .LBB0_1
	ldrb	w11, [x9, x8]
	add	x10, x10, x11, lsl #2
	br	x10
.LBB0_1:                                // %atomicrmw.start
	ldxr	x8, [x0]
	add	x9, x8, x1
	stxr	w10, x9, [x0]
	cbnz	w10, .LBB0_1
	b	.LBB0_6
.LBB0_2:                                // %atomicrmw.start2
	ldxr	x8, [x0]
	add	x9, x8, x1
	stlxr	w10, x9, [x0]
	cbnz	w10, .LBB0_2
	b	.LBB0_6
.LBB0_3:                                // %atomicrmw.start6
	ldaxr	x8, [x0]
	add	x9, x8, x1
	stxr	w10, x9, [x0]
	cbnz	w10, .LBB0_3
	b	.LBB0_6
.LBB0_4:                                // %atomicrmw.start10
	ldaxr	x8, [x0]
	add	x9, x8, x1
	stlxr	w10, x9, [x0]
	cbnz	w10, .LBB0_4
	b	.LBB0_6
.LBB0_5:                                // %atomicrmw.start14
	ldaxr	x8, [x0]
	add	x9, x8, x1
	stlxr	w10, x9, [x0]
	cbnz	w10, .LBB0_5
.LBB0_6:                                // %atomicrmw.end13
	str	x8, [sp, #8]
	mov	x0, x8
	add	sp, sp, #16
	ret
