
(gdb) stepi
0x000055eee90b3510 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3510 <+432>:	test   %bx,%bx

(gdb) stepi
0x000055eee90b3513 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3513 <+435>:	je     0x55eee90b3540 <_ZN9hashbrown3map24HashMap$LT$K$C$V$C$S$GT$6remove17h07f1bbea6c26227cE+480>

(gdb) stepi
0x000055eee90b3540 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3540 <+480>:	add    %rdx,%rax
   
(gdb) stepi
0x000055eee90b3543 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3543 <+483>:	add    $0x10,%rax

(gdb) stepi
0x000055eee90b3547 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3547 <+487>:	add    $0x10,%rdx

(gdb) stepi
0x000055eee90b354b in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b354b <+491>:	pcmpeqb %xmm1,%xmm2

(gdb) stepi
0x000055eee90b354f in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b354f <+495>:	pmovmskb %xmm2,%ecx

(gdb) stepi
0x000055eee90b3553 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3553 <+499>:	test   %cx,%cx
 
(gdb) stepi
0x000055eee90b3556 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3556 <+502>:	jne    0x55eee90b35ea <_ZN9hashbrown3map24HashMap$LT$K$C$V$C$S$GT$6remove17h07f1bbea6c26227cE+650>

(gdb) stepi
0x000055eee90b355c in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b355c <+508>:	jmp    0x55eee90b34fb <_ZN9hashbrown3map24HashMap$LT$K$C$V$C$S$GT$6remove17h07f1bbea6c26227cE+411>

(gdb) stepi
0x000055eee90b34fb in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b34fb <+411>:	and    %rsi,%rax

(gdb) stepi
0x000055eee90b34fe in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b34fe <+414>:	movdqu (%r9,%rax,1),%xmm2

(gdb) stepi
0x000055eee90b3504 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3504 <+420>:	movdqa %xmm0,%xmm3

(gdb) stepi
0x000055eee90b3508 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3508 <+424>:	pcmpeqb %xmm2,%xmm3

(gdb) stepi
0x000055eee90b350c in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b350c <+428>:	pmovmskb %xmm3,%ebx

(gdb) stepi
0x000055eee90b3510 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3510 <+432>:	test   %bx,%bx

(gdb) stepi
0x000055eee90b3513 in hashbrown::map::HashMap$LT$K$C$V$C$S$GT$::remove::h07f1bbea6c26227c ()
0x000055eee90b3513 <+435>:	je     0x55eee90b3540 <_ZN9hashbrown3map24HashMap$LT$K$C$V$C$S$GT$6remove17h07f1bbea6c26227cE+480>
