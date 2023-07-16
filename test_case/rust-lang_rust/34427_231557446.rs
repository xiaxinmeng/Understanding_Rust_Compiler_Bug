 asm
21c21
<     25b8:       ba000070        blt     2780 <_ZN6sigill3foo17hfaa1209845a9b530E+0x214>
---
>     25b8:       ba00006f        blt     277c <_ZN6sigill3foo17hfaa1209845a9b530E+0x210>
26c26
<     25cc:       eb003fc5        bl      124e8 <__rust_allocate>
---
>     25cc:       eb003fbf        bl      124d0 <__rust_allocate>
29c29
<     25d8:       0a00006d        beq     2794 <_ZN6sigill3foo17hfaa1209845a9b530E+0x228>
---
>     25d8:       0a00006b        beq     278c <_ZN6sigill3foo17hfaa1209845a9b530E+0x220>
49c49
< 2628:       23000000        movwcs  r0, #0
---
>     2628:       23000000        movwcs  r0, #0
67c67
<     2670:       eb003fae        bl      12530 <__rust_reallocate>
---
>     2670:       eb003fa8        bl      12518 <__rust_reallocate>
71c71
<     2680:       eb003f98        bl      124e8 <__rust_allocate>
---
>     2680:       eb003f92        bl      124d0 <__rust_allocate>
79c79
<     26a0:       e7a00008        str     r0, [r0, r8]!
---
>     26a0:       e7a01008        str     r1, [r0, r8]!
