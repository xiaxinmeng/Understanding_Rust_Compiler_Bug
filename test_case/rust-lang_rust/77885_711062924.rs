asm
	add	ebx, 15
	and	ebx, -16
	mov	eax, esp
	sub	eax, ebx
	mov	dword ptr [ebp - 368], edx
	mov	dword ptr [ebp - 372], esi
	mov	dword ptr [ebp - 376], edi
	mov	dword ptr [ebp - 380], eax
.LBB87_43:
	mov	eax, dword ptr [ebp - 380]
	cmp	eax, esp
	jl	.LBB87_45
	mov	dword ptr [esp], 0
	sub	esp, 4096
	jmp	.LBB87_43
.LBB87_45:
	mov	eax, dword ptr [ebp - 380]
	mov	esp, eax
