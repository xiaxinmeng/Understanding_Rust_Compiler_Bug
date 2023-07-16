
$ ./bounds
=================================================================
==1945690==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x602000000080 at pc 0x55b483fa152c bp 0x7ffe26149f30 sp 0x7ffe26149f28
READ of size 4 at 0x602000000080 thread T0
    #0 0x55b483fa152b  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x9852b)
    #1 0x55b483fa056b  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x9756b)
    #2 0x55b483fac9c2  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0xa39c2)
    #3 0x55b483fae166  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0xa5166)
    #4 0x55b483fad358  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0xa4358)
    #5 0x55b483fa04e8  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x974e8)
    #6 0x55b483fa195a  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x9895a)
    #7 0x7faf7c9a23d4  (/lib64/libc.so.6+0x223d4)
    #8 0x55b483f1576b  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0xc76b)

0x602000000080 is located 0 bytes to the right of 16-byte region [0x602000000070,0x602000000080)
allocated by thread T0 here:
    #0 0x55b483f7fd0d  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x76d0d)
    #1 0x55b483f9f01e  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x9601e)
    #2 0x55b483f9f483  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x96483)
    #3 0x55b483fa10ff  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x980ff)
    #4 0x55b483fa056b  (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x9756b)

SUMMARY: AddressSanitizer: heap-buffer-overflow (/data/users/jsgf/fbsource-tp2/fbcode/experimental/jsgf/sanibad/bounds+0x9852b)
Shadow bytes around the buggy address:
  0x0c047fff7fc0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c047fff7fd0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c047fff7fe0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c047fff7ff0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x0c047fff8000: fa fa fd fa fa fa 05 fa fa fa 00 00 fa fa 00 00
=>0x0c047fff8010:[fa]fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8020: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8030: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8040: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8050: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c047fff8060: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
Shadow byte legend (one shadow byte represents 8 application bytes):
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
  Shadow gap:              cc
==1945690==ABORTING
