asm
mov	eax, edi
and	eax, 65280
mov	ecx, edi
and	ecx, 16711680
lea	edx, [rsi + rdi]
and	edi, -16777216
add	edi, esi
and	edi, -16777216
add	ecx, esi
and	ecx, 16711680
or	edi, ecx
add	eax, esi
and	eax, 65280
or	edi, eax
movzx	eax, dl
or	eax, edi
ret
