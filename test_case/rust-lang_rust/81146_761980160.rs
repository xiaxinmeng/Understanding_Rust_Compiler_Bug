asm
<x::foo>:
	    push   %rax
	    callq  <x::bar>
	    cmp    $0x2,%eax
	/-- jne    <x::foo+0x17>
	|   callq  <x::println>
	|   mov    $0x2,%eax
	|   xor    %edx,%edx
	\-> pop    %rcx
	    retq   
