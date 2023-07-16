
failures:

---- [ui] src/test/ui/sanitize/address.rs stdout ----

error: error pattern ' 'xs' (line 15) <== Memory access at offset' not found!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a"
stdout: none
--- stderr -------------------------------
=================================================================
==91225==ERROR: AddressSanitizer: stack-buffer-overflow on address 0x7ffecf376470 at pc 0x55e4acc1e4b3 bp 0x7ffecf376410 sp 0x7ffecf376408
READ of size 4 at 0x7ffecf376470 thread T0
    #0 0x55e4acc1e4b2  (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0xce4b2) (BuildId: abb6fae7b1f29cdb9e20c7b3d323f674ad7ca81a)
    #1 0x55e4acc1e272  (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0xce272) (BuildId: abb6fae7b1f29cdb9e20c7b3d323f674ad7ca81a)
    #2 0x55e4acc1e298  (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0xce298) (BuildId: abb6fae7b1f29cdb9e20c7b3d323f674ad7ca81a)
    #3 0x7fe834184433  (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-05a07863c3cc61e0.so+0x97433) (BuildId: fe8980634f2840c6ff3ae7de4ca2fb2dd9c02f35)
    #4 0x55e4acc1e5a7  (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0xce5a7) (BuildId: abb6fae7b1f29cdb9e20c7b3d323f674ad7ca81a)
    #5 0x7fe833d86082  (/lib/x86_64-linux-gnu/libc.so.6+0x24082) (BuildId: 1878e6b475720c7c51969e69ab2d276fae6d1dee)
    #6 0x55e4acb5828d  (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0x828d) (BuildId: abb6fae7b1f29cdb9e20c7b3d323f674ad7ca81a)

Address 0x7ffecf376470 is located in stack of thread T0 at offset 80 in frame
    #0 0x55e4acc1e2ff  (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0xce2ff) (BuildId: abb6fae7b1f29cdb9e20c7b3d323f674ad7ca81a)

  This frame has 2 object(s):
    [32, 40) '' (line 15)
    [64, 80) 'xs' (line 13) <== Memory access at offset 80 overflows this variable
HINT: this may be a false positive if your program uses some custom stack unwind mechanism, swapcontext or vfork
      (longjmp and C++ exceptions *are* supported)
SUMMARY: AddressSanitizer: stack-buffer-overflow (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/address/a+0xce4b2) (BuildId: abb6fae7b1f29cdb9e20c7b3d323f674ad7ca81a) 
Shadow bytes around the buggy address:
  0x100059e66c30: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100059e66c40: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100059e66c50: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100059e66c60: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100059e66c70: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
=>0x100059e66c80: 00 00 00 00 f1 f1 f1 f1 f8 f2 f2 f2 00 00[f3]f3
  0x100059e66c90: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100059e66ca0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100059e66cb0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100059e66cc0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100059e66cd0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
Shadow byte legend (one shadow byte represents 8 application bytes):
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07 
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
==91225==ABORTING
------------------------------------------



failures:
    [ui] src/test/ui/sanitize/address.rs

test result: FAILED. 13499 passed; 1 failed; 98 ignored; 0 measured; 0 filtered out; finished in 98.54s
