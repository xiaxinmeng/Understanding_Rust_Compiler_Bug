 diff
22,23c22,23
< ffff000040080028:     90000008        adrp    x8, ffff000040080000 <_start>
< ffff00004008002c:     91000108        add     x8, x8, #0x0
---
> ffff000040080028:     b0000008        adrp    x8, ffff000040081000 <etext>
> ffff00004008002c:     f9400108        ldr     x8, [x8]
34a35,45
>       ...
> 
> Disassembly of section .got:
> 
> ffff000040081000 <.got>:
> ffff000040081000:     40080000        .inst   0x40080000 ; undefined
> ffff000040081004:     ffff0000        .inst   0xffff0000 ; undefined
> 
> Disassembly of section .rodata:
> 
> ffff000040081008 <srodata>:
