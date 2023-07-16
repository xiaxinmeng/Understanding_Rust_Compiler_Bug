nasm
	mov	dword ptr [rax], 12
	mov	qword ptr [rsp + 8], 42
	mov	qword ptr [rsp + 16], rax
	mov	qword ptr [rsp + 24], 1
	mov	qword ptr [rsp + 32], 1
	mov	dword ptr [rsp + 40], 79 ; This is what `mut_update` becomes.
; Begin of `println!()`.
	lea	rax, [rsp + 8]
	mov	qword ptr [rsp + 88], rax
	lea	r14, [rip + _ZN47_$LT$opti..Type$u20$as$u20$core..fmt..Debug$GT$3fmt17h06d64ac62be9bfa7E]
	mov	qword ptr [rsp + 96], r14
	lea	rbx, [rip + ref.9]
	mov	qword ptr [rsp + 120], rbx
	mov	qword ptr [rsp + 128], 2
	mov	qword ptr [rsp + 136], 0
	lea	rax, [rsp + 88]
	mov	qword ptr [rsp + 152], rax
	mov	qword ptr [rsp + 160], 1
.Ltmp0:
	lea	rdi, [rsp + 120]
	call	_ZN3std2io5stdio6_print17h4d11beeeb2d08206E@PLT
; End of `println!()`.
.Ltmp1:
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rsp + 48], rax
	mov	dword ptr [rsp + 80], 112
	mov	rax, qword ptr [rsp + 32]
	mov	qword ptr [rsp + 72], rax
	movups	xmm0, xmmword ptr [rsp + 16]
	movups	xmmword ptr [rsp + 56], xmm0
; Yep. Everything is still copied for no reason.
