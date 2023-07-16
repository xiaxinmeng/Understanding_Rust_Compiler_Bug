asm
// %bb.0:                               // %start
	sub	sp, sp, #128            // =128
	str	x30, [sp, #112]         // 8-byte Folded Spill
	.cfi_def_cfa_offset 128
	.cfi_offset w30, -16
	bl	_ZN3std2fs10DirBuilder3new17h4be627efa14f8660E
	str	w0, [sp, #104]
	and	w0, w1, #0x1
	strb	w0, [sp, #108]
