
	.vbyte	4, L..tmp0-L..func_begin0       # >> Call Site 1 <<
	.vbyte	4, L..tmp1-L..tmp0              #   Call between L..tmp0 and L..tmp1
	.vbyte	4, L..tmp2-L..func_begin0       #     jumps to L..tmp2
	.byte	1                               #   On action: 1
