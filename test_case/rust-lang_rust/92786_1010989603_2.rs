
// %bb.0:
	sub	sp, sp, #32                     // =32
	.cfi_def_cfa_offset 32
	str	x2, [sp, #8]                    // 8-byte Folded Spill
	str	x1, [sp, #16]                   // 8-byte Folded Spill
	str	x0, [sp, #24]                   // 8-byte Folded Spill
	b	.LBB0_1
.LBB0_1:                                // %loop
                                        // =>This Inner Loop Header: Depth=1
	ldr	x9, [sp, #8]                    // 8-byte Folded Reload
	ldr	x10, [sp, #16]                  // 8-byte Folded Reload
	ldr	x8, [sp, #24]                   // 8-byte Folded Reload
	ldr	x8, [x8]
	ucvtf	s0, x8
	str	s0, [x10]
                                        // kill: def $w8 killed $w8 killed $x8
	str	w8, [x9]
	b	.LBB0_1
