asm
before (11-10):
	movq	(%r10), %rdx
	addq	$8, %r10
	movl	%edx, %edi
	shrl	$3, %edi // hash >> 3
	xorl	%edx, %edi
	movl	%edi, %eax
	shrl	%eax // hash >> 1 !? see below
	shrl	$24, %edi // hash >> 24 !? see below
	xorl	%eax, %edi
	andl	$31, %edi // hash & 31
	xorl	%eax, %eax
	cmpq	%rdx, (%r8,%rdi,8)
	sete	%al
	addl	%eax, %ecx
	cmpq	%r10, %r9


after (11-11):

	movq	(%r11,%rdx,8), %rcx
	movl	%ecx, %esi
	shrl	$3, %esi // hash >> 3
	xorl	%ecx, %esi
	rorl	$1, %esi // hash.rotate_right(1)
	movl	%esi, %eax
	shrl	$23, %eax // hash >> 23
	xorl	%esi, %eax
	andl	$31, %eax // hash & 31
	xorl	%esi, %esi
	cmpq	%rcx, (%r9,%rax,8)
	sete	%sil
	addl	%esi, %edi
	addq	$1, %rdx
	cmpq	%rdx, %r8
