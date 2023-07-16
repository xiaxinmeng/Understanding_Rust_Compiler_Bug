asm
atomicaarch64::main:
Lloh2:
 adrp    x8, __ZN13atomicaarch642ID17hb9c7e370ca9094e2E@PAGE
Lloh3:
 add     x8, x8, __ZN13atomicaarch642ID17hb9c7e370ca9094e2E@PAGEOFF
 mov     w9, #1
 ldadd   x9, x8, [x8]
 ret
