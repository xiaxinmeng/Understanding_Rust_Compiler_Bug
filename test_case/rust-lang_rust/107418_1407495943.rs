nasm

playground::reg_name:
	movzx	ecx, dil
	shl	rcx, 4
	lea	rdx, [rip + .L__unnamed_1]
	mov	rax, qword ptr [rcx + rdx]
	mov	rdx, qword ptr [rcx + rdx + 8]
	ret
