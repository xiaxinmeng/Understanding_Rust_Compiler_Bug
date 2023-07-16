
playground::non_zero:
	bsr	eax, edi
	xor	eax, 31
	ret

playground::non_zero_ind:
	bsr	eax, dword ptr [rdi]
	xor	eax, 31
	ret
