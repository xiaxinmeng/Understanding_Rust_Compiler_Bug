x86asm
(gdb) p rc
$1 = Rc(strong=1, weak=1) = {
  value = 5,
  strong = 1,
  weak = 1
}

(gdb) x/xg &rc
0x7fffffffe120:	0x000055555559b9d0

(gdb) x/3xg 0x000055555559b9d0
0x55555559b9d0:	0x0000000000000001	0x0000000000000002
0x55555559b9e0:	0x0000000000000005
