
Disassembly of section .text._ZN17compiler_builtins9arm_linux29__sync_val_compare_and_swap_417h4daedd6aaff95c3cE:

00000000 <_ZN17compiler_builtins9arm_linux29__sync_val_compare_and_swap_417h4daedd6aaff95c3cE>:
   0:	e92d48f0 	push	{r4, r5, r6, r7, fp, lr}
   4:	e59f7034 	ldr	r7, [pc, #52]	; 40 <_ZN17compiler_builtins9arm_linux29__sync_val_compare_and_swap_417h4daedd6aaff95c3cE+0x40>
   8:	e1a05002 	mov	r5, r2
   c:	e1a04001 	mov	r4, r1
  10:	e1a06000 	mov	r6, r0
  14:	e5960000 	ldr	r0, [r6]
  18:	e1500004 	cmp	r0, r4
  1c:	1a000006 	bne	3c <_ZN17compiler_builtins9arm_linux29__sync_val_compare_and_swap_417h4daedd6aaff95c3cE+0x3c>
  20:	e1a00004 	mov	r0, r4
  24:	e1a01005 	mov	r1, r5
  28:	e1a02006 	mov	r2, r6
  2c:	e12fff37 	blx	r7
  30:	e3500000 	cmp	r0, #0
  34:	1afffff6 	bne	14 <_ZN17compiler_builtins9arm_linux29__sync_val_compare_and_swap_417h4daedd6aaff95c3cE+0x14>
  38:	e1a00004 	mov	r0, r4
  3c:	e8bd88f0 	pop	{r4, r5, r6, r7, fp, pc}
  40:	ffff0fc0 			; <UNDEFINED> instruction: 0xffff0fc0

Disassembly of section .ARM.exidx.text._ZN17compiler_builtins9arm_linux29__sync_val_compare_and_swap_417h4daedd6aaff95c3cE:

00000000 <.ARM.exidx.text._ZN17compiler_builtins9arm_linux29__sync_val_compare_and_swap_417h4daedd6aaff95c3cE>:
   0:	00000000 	andeq	r0, r0, r0
   4:	00000001 	andeq	r0, r0, r1

