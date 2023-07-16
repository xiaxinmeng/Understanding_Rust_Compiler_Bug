asm
// %bb.0:                               // %start
	sub	sp, sp, #144            // =144
	str	x30, [sp, #128]         // 8-byte Folded Spill
	.cfi_def_cfa_offset 144
	.cfi_offset w30, -16
	adrp	x1, _ZN56_$LT$std..fs..DirBuilder$u20$as$u20$core..fmt..Debug$GT$3fmt17h81b1b3fa607a0cf5E
	add	x1, x1, :lo12:_ZN56_$LT$std..fs..DirBuilder$u20$as$u20$core..fmt..Debug$GT$3fmt17h81b1b3fa607a0cf5E
	adrp	x8, .Lbyte_str.3
	add	x8, x8, :lo12:.Lbyte_str.3
	adrp	x9, .Lbyte_str.4
	add	x9, x9, :lo12:.Lbyte_str.4
	str	x1, [sp, #40]           // 8-byte Folded Spill
	str	x8, [sp, #32]           // 8-byte Folded Spill
	str	x9, [sp, #24]           // 8-byte Folded Spill
	bl	_ZN3std2fs10DirBuilder3new17h4be627efa14f8660E
                                        // implicit-def: %x8
	mov	w9, w0
	bfxil	x8, x9, #0, #32
	mov	w9, w1
	bfi	x8, x9, #32, #1
	str	x8, [sp, #120]
