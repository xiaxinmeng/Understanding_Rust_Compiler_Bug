asm
portable_simd_test::mask_all:
	movi v0.2d, #0xffffffffffffffff
	ldr q1, [x0]
	cmgt v0.16b, v1.16b, v0.16b
	umaxv b0, v0.16b
	fmov w8, s0
	mvn w8, w8
	and w0, w8, #0x1
	ret
