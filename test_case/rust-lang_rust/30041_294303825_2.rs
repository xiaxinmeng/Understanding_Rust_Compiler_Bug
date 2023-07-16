asm
good1:
    retq

good2:
    retq

good3:
    retq

good4:
    retq

bad1:
    cmpb   $0x0,(%rdi)
    je     8 <bad1+0x8>
    movb   $0x1,(%rdi)
    retq

bad2:
    cmpb   $0x0,(%rdi)
    je     8 <bad2+0x8>
    movb   $0x1,(%rdi)
    retq

bad3:
    retq
