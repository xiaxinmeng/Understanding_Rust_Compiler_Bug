
0000006e <__vector3>:
  6e:	1f 92       	push	r1
  70:	0f 92       	push	r0
  72:	00 90 5f 00 	lds	r0, 0x005F	; 0x80005f <__EEPROM_REGION_LENGTH__+0x7f005f>
  76:	0f 92       	push	r0
  78:	11 24       	eor	r1, r1
  7a:	2f 93       	push	r18
  7c:	3f 93       	push	r19
  7e:	4f 93       	push	r20
  80:	5f 93       	push	r21
  82:	6f 93       	push	r22
  84:	7f 93       	push	r23
  86:	8f 93       	push	r24
  88:	9f 93       	push	r25
  8a:	af 93       	push	r26
  8c:	bf 93       	push	r27
  8e:	ef 93       	push	r30
  90:	ff 93       	push	r31
  92:	cf 93       	push	r28
  94:	df 93       	push	r29
  96:	cd b7       	in	r28, 0x3d	; 61
  98:	dd 27       	eor	r29, r29
  9a:	ca df       	rcall	.-108    	; 0x30 <never_inlined>
  9c:	00 00       	nop
  9e:	df 91       	pop	r29
  a0:	cf 91       	pop	r28
  a2:	ff 91       	pop	r31
  a4:	ef 91       	pop	r30
  a6:	bf 91       	pop	r27
  a8:	af 91       	pop	r26
  aa:	9f 91       	pop	r25
  ac:	8f 91       	pop	r24
  ae:	7f 91       	pop	r23
  b0:	6f 91       	pop	r22
  b2:	5f 91       	pop	r21
  b4:	4f 91       	pop	r20
  b6:	3f 91       	pop	r19
  b8:	2f 91       	pop	r18
  ba:	0f 90       	pop	r0
  bc:	00 92 5f 00 	sts	0x005F, r0	; 0x80005f <__EEPROM_REGION_LENGTH__+0x7f005f>
  c0:	0f 90       	pop	r0
  c2:	1f 90       	pop	r1
  c4:	18 95       	reti
