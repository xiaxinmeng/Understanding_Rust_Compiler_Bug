asm
portable_simd_test::select:
	ldr q1, [x0]
	ldr q0, [x1]
	ldr q2, [x2]
	shl v1.4s, v1.4s, #31
	cmlt v1.4s, v1.4s, #0
	bif v0.16b, v2.16b, v1.16b
	str q0, [x8]
	ret
