asm
mov	rax, rdi
add	r9b, sil
add	dl, byte ptr [rsp + 8]
add	cl, byte ptr [rsp + 16]
add	r8b, byte ptr [rsp + 24]
mov	byte ptr [rdi + 3], r8b
mov	byte ptr [rdi + 2], cl
mov	byte ptr [rdi + 1], dl
mov	byte ptr [rdi], r9b
ret
