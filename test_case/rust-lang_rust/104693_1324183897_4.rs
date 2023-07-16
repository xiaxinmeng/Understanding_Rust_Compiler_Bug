asm
portable_simd_test::mask_all:
	.cfi_startproc
	sub sp, sp, #16
	.cfi_def_cfa_offset 16

	ldr q0, [x0]

	mov w8, #65535

	umov w9, v0.b[1]
	umov w11, v0.b[2]
	umov w10, v0.b[0]
	umov w12, v0.b[3]
	umov w13, v0.b[4]
	umov w14, v0.b[5]
	and w9, w9, #0x1
	and w11, w11, #0x1
	and w10, w10, #0x1
	and w12, w12, #0x1
	and w13, w13, #0x1
	and w14, w14, #0x1
	bfi w10, w9, #1, #1
	umov w9, v0.b[6]
	bfi w10, w11, #2, #1
	umov w11, v0.b[7]
	bfi w10, w12, #3, #1
	umov w12, v0.b[8]
	bfi w10, w13, #4, #1
	umov w13, v0.b[9]
	and w9, w9, #0x1
	bfi w10, w14, #5, #1
	umov w14, v0.b[10]
	and w11, w11, #0x1
	orr w9, w10, w9, lsl #6
	umov w10, v0.b[11]
	and w12, w12, #0x1
	orr w9, w9, w11, lsl #7
	umov w11, v0.b[12]
	and w13, w13, #0x1
	orr w9, w9, w12, lsl #8
	umov w12, v0.b[13]
	and w14, w14, #0x1
	orr w9, w9, w13, lsl #9
	umov w13, v0.b[14]
	and w10, w10, #0x1
	orr w9, w9, w14, lsl #10
	and w11, w11, #0x1
	orr w9, w9, w10, lsl #11
	and w10, w12, #0x1
	umov w12, v0.b[15]
	orr w9, w9, w11, lsl #12
	and w11, w13, #0x1
	orr w9, w9, w10, lsl #13
	orr w9, w9, w11, lsl #14
	orr w9, w9, w12, lsl #15
	bics wzr, w8, w9
	cset w0, eq

	add sp, sp, #16
	.cfi_def_cfa_offset 0
	ret
