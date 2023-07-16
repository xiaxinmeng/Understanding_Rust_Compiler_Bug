
ctzdi2.o:     file format elf64-tradlittlemips


Disassembly of section .text:

0000000000000000 <__ctzdi2>:
   0:   67bdffe0        daddiu  sp,sp,-32
   4:   00041000        sll     v0,a0,0x0
   8:   ffbc0010        sd      gp,16(sp)
   c:   3c1c0000        lui     gp,0x0
  10:   ffb00008        sd      s0,8(sp)
  14:   0399e02d        daddu   gp,gp,t9
  18:   2c500001        sltiu   s0,v0,1
  1c:   679c0000        daddiu  gp,gp,0
  20:   0010802f        dnegu   s0,s0
  24:   00102827        nor     a1,zero,s0
  28:   df990000        ld      t9,0(gp)
  2c:   0004203f        dsra32  a0,a0,0x0
  30:   00901824        and     v1,a0,s0
  34:   00a22024        and     a0,a1,v0
  38:   ffbf0018        sd      ra,24(sp)
  3c:   0320f809        jalr    t9
  40:   00642025        or      a0,v1,a0
  44:   dfbf0018        ld      ra,24(sp)
  48:   32100020        andi    s0,s0,0x20
  4c:   02021021        addu    v0,s0,v0
  50:   dfbc0010        ld      gp,16(sp)
  54:   dfb00008        ld      s0,8(sp)
  58:   03e00008        jr      ra
  5c:   67bd0020        daddiu  sp,sp,32
