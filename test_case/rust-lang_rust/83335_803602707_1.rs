  diff
7,19c7,27
< 0000000000210158 <_start>:
<   210158:     90000000        adrp    x0, 210000 <_start-0x158>
<   21015c:     9100001f        mov     sp, x0
<   210160:     14000001        b       210164 <main_start>
< 
< 0000000000210164 <main_start>:
<   210164:     d0000088        adrp    x8, 222000 <data_block2>
<   210168:     52800549        mov     w9, #0x2a                       // #42
<   21016c:     39000109        strb    w9, [x8]
<   210170:     b0000089        adrp    x9, 221000 <data_block>
<   210174:     3940013f        ldrb    wzr, [x9]
<   210178:     3940011f        ldrb    wzr, [x8]
<   21017c:     17fffffe        b       210174 <main_start+0x10>
---
> 00000000002101c8 <_start>:
>   2101c8:     90000000        adrp    x0, 210000 <_start-0x1c8>
>   2101cc:     9100001f        mov     sp, x0
>   2101d0:     14000001        b       2101d4 <main_start>
> 
> 00000000002101d4 <main_start>:
>   2101d4:     b0000088        adrp    x8, 221000 <main_start+0x10e2c>
>   2101d8:     f940fd08        ldr     x8, [x8, #504]
>   2101dc:     52800549        mov     w9, #0x2a                       // #42
>   2101e0:     39000109        strb    w9, [x8]
>   2101e4:     90000089        adrp    x9, 220000 <main_start+0xfe2c>
>   2101e8:     f940fd29        ldr     x9, [x9, #504]
>   2101ec:     3940013f        ldrb    wzr, [x9]
>   2101f0:     3940011f        ldrb    wzr, [x8]
>   2101f4:     17fffffe        b       2101ec <main_start+0x18>
> 
> Disassembly of section .got:
> 
> 00000000002201f8 <.got>:
>   2201f8:     00231000        .inst   0x00231000 ; NYI
>   2201fc:     00000000        udf     #0
23c31
< 0000000000221000 <data_block>:
---
> 0000000000231000 <data_block>:
26c34
< 0000000000222000 <data_block2>:
---
> 0000000000232000 <data_block2>:
40c48
<   20: 00210164        .inst   0x00210164 ; NYI
---
>   20: 002101d4        .inst   0x002101d4 ; NYI
42c50
<   28: 0000001c        udf     #28
---
>   28: 00000024        udf     #36
51c59
<    c: 302e3131        adr     x17, 5c631 <_start-0x1b3b27>
---
>    c: 302e3231        adr     x17, 5c651 <_start-0x1b3b77>
