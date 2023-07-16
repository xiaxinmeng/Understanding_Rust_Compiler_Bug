
// %bb.0:
	sub	sp, sp, #32                     // =32
	.cfi_def_cfa_offset 32
	str	x0, [sp, #8]                    // 8-byte Folded Spill
	str	x1, [sp, #16]                   // 8-byte Folded Spill
	str	x2, [sp, #24]                   // 8-byte Folded Spill
.LBB0_1:                                // %loop
                                        // =>This Inner Loop Header: Depth=1
	ldr	x8, [sp, #24]                   // 8-byte Folded Reload
	ldr	x9, [sp, #16]                   // 8-byte Folded Reload
	ldr	x10, [sp, #8]                   // 8-byte Folded Reload
	ldr	d0, [x10]
	mrs	s0, NZCV
	mrs	x10, NZCV
	ucvtf	s1, x10
	str	s1, [x9]
	str	s0, [x8]
	b	.LBB0_1
