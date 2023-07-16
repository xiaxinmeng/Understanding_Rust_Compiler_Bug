
  402af0:	d2a00000 	movz	x0, #0x0, lsl #16
  402af4:	f2800400 	movk	x0, #0x20 <-- THIS SHOULD BE 0x10
  402af8:	d503201f 	nop
  402afc:	d503201f 	nop
  402b00:	d53bd048 	mrs	x8, tpidr_el0
  402b04:	8b000108 	add	x8, x8, x0
  402b08:	f9400d08 	ldr	x8, [x8, #24]
