asm
# %bb.5:                                # %bb3
    [...]
	pushl	$4
	popl	%esi                <-- esi = 4
    [...]
	leal	72(%esp), %eax      <-- eax = esp + 72
	orl	%eax, %esi              <-- eax = (esp + 72) | 4
