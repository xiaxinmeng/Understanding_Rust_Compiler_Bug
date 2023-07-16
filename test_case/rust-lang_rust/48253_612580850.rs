assembly
playground::to_color_fast:
	movq	%rdi, %rax
	andb	$1, %al
	retq
.set playground::to_color, playground::to_color_fast
