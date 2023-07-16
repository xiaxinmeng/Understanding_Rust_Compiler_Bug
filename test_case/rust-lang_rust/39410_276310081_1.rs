asm
	.text
	.file	"armv8_none_rbit.cgu-0.rs"
	.section	.text.rbit_u64,"ax",@progbits
	.globl	rbit_u64
	.p2align	2
	.type	rbit_u64,@function
rbit_u64:
	lsl	x7, x0, #61
	and	x7, x7, #0x4000000000000000
	lsr	x6, x0, #2
	bfi	x7, x0, #63, #1
	lsr	x4, x0, #3
	bfi	x7, x6, #61, #1
	lsr	x5, x0, #4
	bfi	x7, x4, #60, #1
	lsr	x3, x0, #5
	lsl	x6, x0, #51
	bfi	x7, x5, #59, #1
	lsl	x5, x0, #49
	and	x6, x6, #0x200000000000000
	bfi	x7, x3, #58, #1
	orr		x6, x7, x6
	lsl	x7, x0, #47
	and	x5, x5, #0x100000000000000
	orr		x5, x6, x5
	lsl	x6, x0, #45
	and	x7, x7, #0x80000000000000
	orr		x5, x5, x7
	lsl	x7, x0, #43
	and	x6, x6, #0x40000000000000
	orr		x5, x5, x6
	lsl	x6, x0, #41
	and	x7, x7, #0x20000000000000
	orr		x5, x5, x7
	lsl	x7, x0, #39
	and	x6, x6, #0x10000000000000
	orr		x5, x5, x6
	lsl	x6, x0, #37
	and	x7, x7, #0x8000000000000
	orr		x5, x5, x7
	lsl	x7, x0, #35
	and	x6, x6, #0x4000000000000
	orr		x5, x5, x6
	lsl	x6, x0, #33
	and	x7, x7, #0x2000000000000
	orr		x5, x5, x7
	lsl	x7, x0, #31
	and	x6, x6, #0x1000000000000
	orr		x5, x5, x6
	lsl	x6, x0, #29
	and	x7, x7, #0x800000000000
	orr		x5, x5, x7
	lsl	x7, x0, #27
	and	x6, x6, #0x400000000000
	orr		x5, x5, x6
	lsl	x6, x0, #25
	and	x7, x7, #0x200000000000
	orr		x5, x5, x7
	lsl	x7, x0, #23
	and	x6, x6, #0x100000000000
	orr		x5, x5, x6
	lsl	x6, x0, #21
	and	x7, x7, #0x80000000000
	orr		x5, x5, x7
	lsl	x7, x0, #19
	and	x6, x6, #0x40000000000
	orr		x5, x5, x6
	lsl	x6, x0, #17
	and	x7, x7, #0x20000000000
	orr		x5, x5, x7
	lsl	x7, x0, #15
	and	x6, x6, #0x10000000000
	orr		x5, x5, x6
	lsl	x6, x0, #13
	and	x7, x7, #0x8000000000
	orr		x5, x5, x7
	lsl	x7, x0, #11
	and	x6, x6, #0x4000000000
	orr		x5, x5, x6
	lsl	x6, x0, #9
	and	x7, x7, #0x2000000000
	orr		x5, x5, x7
	lsl	x7, x0, #7
	and	x6, x6, #0x1000000000
	orr		x5, x5, x6
	lsl	x6, x0, #5
	and	x7, x7, #0x800000000
	orr		x5, x5, x7
	lsl	x7, x0, #3
	and	x6, x6, #0x400000000
	orr		x5, x5, x6
	lsl	x6, x0, #1
	and	x7, x7, #0x200000000
	orr		x5, x5, x7
	lsr	x7, x0, #1
	and	x6, x6, #0x100000000
	orr		x5, x5, x6
	and	x7, x7, #0x80000000
	orr		x5, x5, x7
	and	x4, x4, #0x40000000
	lsr	x2, x0, #7
	orr		x4, x5, x4
	and	x3, x3, #0x20000000
	lsr	x1, x0, #9
	orr		x3, x4, x3
	and	x2, x2, #0x10000000
	lsr	x18, x0, #11
	orr		x2, x3, x2
	and	x1, x1, #0x8000000
	lsr	x17, x0, #13
	orr		x1, x2, x1
	and	x18, x18, #0x4000000
	lsr	x16, x0, #15
	orr		x18, x1, x18
	and	x17, x17, #0x2000000
	lsr	x15, x0, #17
	orr		x17, x18, x17
	and	x16, x16, #0x1000000
	lsr	x14, x0, #19
	orr		x16, x17, x16
	and	x15, x15, #0x800000
	lsr	x13, x0, #21
	orr		x15, x16, x15
	and	x14, x14, #0x400000
	lsr	x12, x0, #23
	orr		x14, x15, x14
	and	x13, x13, #0x200000
	lsr	x11, x0, #25
	orr		x13, x14, x13
	and	x12, x12, #0x100000
	lsr	x10, x0, #27
	orr		x12, x13, x12
	and	x11, x11, #0x80000
	lsr	x9, x0, #29
	orr		x11, x12, x11
	and	x10, x10, #0x40000
	lsr	x8, x0, #31
	orr		x10, x11, x10
	and	x9, x9, #0x20000
	lsr	x6, x0, #33
	and	x8, x8, #0x10000
	orr		x9, x10, x9
	lsr	x7, x0, #35
	and	x6, x6, #0x8000
	orr		x8, x9, x8
	lsr	x5, x0, #37
	and	x7, x7, #0x4000
	orr		x8, x8, x6
	lsr	x4, x0, #39
	and	x5, x5, #0x2000
	orr		x8, x8, x7
	lsr	x3, x0, #41
	and	x4, x4, #0x1000
	orr		x8, x8, x5
	lsr	x2, x0, #43
	and	x3, x3, #0x800
	orr		x8, x8, x4
	lsr	x1, x0, #45
	and	x2, x2, #0x400
	orr		x8, x8, x3
	lsr	x18, x0, #47
	and	x1, x1, #0x200
	orr		x8, x8, x2
	lsr	x17, x0, #49
	and	x18, x18, #0x100
	orr		x8, x8, x1
	lsr	x16, x0, #51
	and	x17, x17, #0x80
	orr		x8, x8, x18
	lsr	x15, x0, #53
	and	x16, x16, #0x40
	orr		x8, x8, x17
	lsr	x14, x0, #55
	and	x15, x15, #0x20
	orr		x8, x8, x16
	lsr	x13, x0, #57
	and	x14, x14, #0x10
	orr		x8, x8, x15
	lsr	x12, x0, #59
	and	x13, x13, #0x8
	orr		x8, x8, x14
	lsr	x11, x0, #61
	and	x12, x12, #0x4
	orr		x8, x8, x13
	orr		x8, x8, x12
	and	x9, x11, #0x2
	orr		x8, x8, x9
	orr	x0, x8, x0, lsr #63
	ret
.Lfunc_end0:
	.size	rbit_u64, .Lfunc_end0-rbit_u64


	.section	".note.GNU-stack","",@progbits
