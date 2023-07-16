 asm
    1e17:       dd 44 24 5c             fldl   0x5c(%esp) ; load the result of parse
    1e1b:       d8 86 a7 20 00 00       fadds  0x20a7(%esi) ; add 5.0 from memory
    1e21:       dd 54 24 10             fstl   0x10(%esp) ; store the result
    1e25:       8d 44 24 10             lea    0x10(%esp),%eax
    1e29:       89 44 24 4c             mov    %eax,0x4c(%esp)
    1e2d:       8d 86 cf 20 00 00       lea    0x20cf(%esi),%eax
    1e33:       89 44 24 50             mov    %eax,0x50(%esp)
    1e37:       dd 86 9f 20 00 00       fldl   0x209f(%esi) ; load 8.1415
    1e3d:       d9 c9                   fxch   %st(1) ; exchange the two top elements of the FP stack (why???)
    1e3f:       da e9                   fucompp ; compare the two top elements and pop both
