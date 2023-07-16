asm
portable_simd_test::select:
	ldr q0, [x0]
	ldr q1, [x1]
	ldr q2, [x2]
	cmlt v0.4s, v0.4s, #0
	bsl v0.16b, v1.16b, v2.16b
	str q0, [x8]
	ret
