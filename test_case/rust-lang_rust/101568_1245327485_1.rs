asm
	push rbx
	lea rsi, [rip + .L__unnamed_1] ; "Hello, "
	mov rbx, qword ptr [rip + write@GOTPCREL]
	mov edx, 7
	mov edi, 1
	call rbx

	lea rsi, [rip + .L__unnamed_2]; "world"
	mov edx, 5
	mov edi, 1
	call rbx

	lea rsi, [rip + .L__unnamed_3]; "!"
	mov edx, 1
	mov edi, 1
	mov rax, rbx
	pop rbx
	jmp rax
