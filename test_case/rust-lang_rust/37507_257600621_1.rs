
  (gdb) r
  Starting program: /root/kinton-yeelight 
  warning: Unable to find dynamic linker breakpoint function.
  GDB will be unable to debug shared library initializers
  and track explicitly loaded dynamic code.
  
  Program received signal SIGILL, Illegal instruction.
  0x5556269c in ?? ()
  (gdb) bt
  #0  0x5556269c in ?? ()
  #1  0x55565fdc in _$LT$str$u20$as$u20$std..net..addr..ToSocketAddrs$GT$::to_socket_addrs::h7d1fe6bb64ce9c00 ()
  #2  0x55560cec in _ftext ()
  