
0000000000008d50 <tests::main>:
    8d50: 09 00 4c 3c  	addis 2, 12, 9
    8d54: b0 f1 42 38  	addi 2, 2, -3664
    8d58: a6 02 08 7c  	mflr 0
    8d5c: 10 00 01 f8  	std 0, 16(1)
    8d60: 51 ff 21 f8  	stdu 1, -176(1)
    8d64: fd ff 62 3c  	addis 3, 2, -3
    8d68: 6c c4 63 38  	addi 3, 3, -15252
    8d6c: 4d fb ff 4b  	bl 0x88b8
    8d70: 00 00 00 60  	nop
    8d74: 60 00 81 f8  	std 4, 96(1)
    8d78: 68 00 61 f8  	std 3, 104(1)
    8d7c: 60 00 61 e8  	ld 3, 96(1)
    8d80: 68 00 81 e8  	ld 4, 104(1)
    8d84: a0 00 81 f8  	std 4, 160(1)
    8d88: a8 00 61 f8  	std 3, 168(1)
    8d8c: a0 00 c1 38  	addi 6, 1, 160
    8d90: ff ff 62 3c  	addis 3, 2, -1
    8d94: c0 48 83 38  	addi 4, 3, 18624
    8d98: 70 00 61 38  	addi 3, 1, 112
    8d9c: 02 00 a0 38  	li 5, 2
    8da0: 01 00 e0 38  	li 7, 1
    8da4: c5 fd ff 4b  	bl 0x8b68
    8da8: 00 00 00 60  	nop
    8dac: 70 00 61 38  	addi 3, 1, 112
    8db0: 59 77 01 48  	bl 0x20508
    8db4: 00 00 00 60  	nop
    8db8: b0 00 21 38  	addi 1, 1, 176
    8dbc: 10 00 01 e8  	ld 0, 16(1)
    8dc0: a6 03 08 7c  	mtlr 0
    8dc4: 20 00 80 4e  	blr
		...
    8dd4: 00 00 00 60  	nop
    8dd8: 00 00 00 60  	nop
    8ddc: 00 00 00 60  	nop
