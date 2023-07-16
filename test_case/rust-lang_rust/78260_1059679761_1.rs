
00000042 <__vector2>:
  42:	1f 92       	push	r1
  44:	0f 92       	push	r0
  46:	00 90 5f 00 	lds	r0, 0x005F	; 0x80005f <__EEPROM_REGION_LENGTH__+0x7f005f>
  4a:	0f 92       	push	r0
  4c:	11 24       	eor	r1, r1
  4e:	8f 93       	push	r24
  50:	cf 93       	push	r28
  52:	df 93       	push	r29
  54:	cd b7       	in	r28, 0x3d	; 61
  56:	dd 27       	eor	r29, r29
  58:	8a e2       	ldi	r24, 0x2A	; 42
  5a:	00 00       	nop
  5c:	df 91       	pop	r29
  5e:	cf 91       	pop	r28
  60:	8f 91       	pop	r24
  62:	0f 90       	pop	r0
  64:	00 92 5f 00 	sts	0x005F, r0	; 0x80005f <__EEPROM_REGION_LENGTH__+0x7f005f>
  68:	0f 90       	pop	r0
  6a:	1f 90       	pop	r1
  6c:	18 95       	reti
