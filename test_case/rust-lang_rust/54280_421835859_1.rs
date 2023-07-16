 console
$ cargo objdump --target thumbv6m-none-eabi --lib --release -- -d
Disassembly of section .text.foo:
foo:
       0:       80 b5   push    {r7, lr}
       2:       bf f3 5f 8f     dmb     sy
       6:       01 21   movs    r1, #1
       8:       ff f7 fe ff     bl      #-4
       c:       bf f3 5f 8f     dmb     sy
      10:       80 bd   pop     {r7, pc}
