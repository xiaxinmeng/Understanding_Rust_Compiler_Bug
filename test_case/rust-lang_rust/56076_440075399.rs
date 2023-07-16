
jonathan:~/git/svpn/target/riscv32imac-unknown-none-elf/debug/incremental/svpn-dceina0o09bg/s-f6tw7h3muc-2szsjf-25qjmb4xljb7i$ riscv32-unknown-elf-objdump -d 33p0vbxbrh75p6mg.o 

33p0vbxbrh75p6mg.o:     file format elf32-littleriscv


Disassembly of section .text.rust_eh_personality:

00000000 <rust_eh_personality>:
   0:	8082                	ret

Disassembly of section .text.rust_begin_unwind:

00000000 <rust_begin_unwind>:
   0:	1141                	addi	sp,sp,-16
   2:	c62a                	sw	a0,12(sp)
   4:	c42a                	sw	a0,8(sp)
   6:	a009                	j	8 <rust_begin_unwind+0x8>
   8:	a001                	j	8 <rust_begin_unwind+0x8>

Disassembly of section .text._ZN4svpn5start17h4b2c7e44f872bbbeE:

00000000 <_ZN4svpn5start17h4b2c7e44f872bbbeE>:
   0:	1101                	addi	sp,sp,-32
   2:	ce06                	sw	ra,28(sp)
   4:	567d                	li	a2,-1
   6:	00a626b3          	slt	a3,a2,a0
   a:	50550713          	addi	a4,a0,1285
   e:	00e62633          	slt	a2,a2,a4
  12:	8e35                	xor	a2,a2,a3
  14:	00c03633          	snez	a2,a2
  18:	8e75                	and	a2,a2,a3
  1a:	cc2e                	sw	a1,24(sp)
  1c:	ca2a                	sw	a0,20(sp)
  1e:	c82a                	sw	a0,16(sp)
  20:	c62e                	sw	a1,12(sp)
  22:	c43a                	sw	a4,8(sp)
  24:	e611                	bnez	a2,30 <_ZN4svpn5start17h4b2c7e44f872bbbeE+0x30>
  26:	a009                	j	28 <_ZN4svpn5start17h4b2c7e44f872bbbeE+0x28>
  28:	4522                	lw	a0,8(sp)
  2a:	40f2                	lw	ra,28(sp)
  2c:	6105                	addi	sp,sp,32
  2e:	8082                	ret
  30:	00000537          	lui	a0,0x0
  34:	00050513          	mv	a0,a0
  38:	00000097          	auipc	ra,0x0
  3c:	000080e7          	jalr	ra
  40:	00000097          	auipc	ra,0x0
  44:	000080e7          	jalr	ra

Disassembly of section .text.main:

00000000 <main>:
   0:	1101                	addi	sp,sp,-32
   2:	ce06                	sw	ra,28(sp)
   4:	cc2a                	sw	a0,24(sp)
   6:	ca2e                	sw	a1,20(sp)
   8:	00000097          	auipc	ra,0x0
   c:	000080e7          	jalr	ra
  10:	45d2                	lw	a1,20(sp)
  12:	4662                	lw	a2,24(sp)
  14:	c832                	sw	a2,16(sp)
  16:	c62e                	sw	a1,12(sp)
  18:	40f2                	lw	ra,28(sp)
  1a:	6105                	addi	sp,sp,32
  1c:	8082                	ret
