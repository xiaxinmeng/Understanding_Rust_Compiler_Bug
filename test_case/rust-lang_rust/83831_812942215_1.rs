asm
_ZN9inline_ip16ipv4_bitand_mask17h21a1d58399eeef05E:
	movl	%ecx, %eax
	andl	%edx, %eax
	retq

_ZN9inline_ip16ipv6_bitand_mask17h84a3a89719ccdfc1E:
	movq	%rcx, %rax
	andq	%r8, %rax
	andq	%r9, %rdx
	retq
