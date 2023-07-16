
$ nm ./build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-f279a465a88fb38f.so | rg 'eh_person|panic_old'
00000000003491a0 d DW.ref.rust_eh_personality
00000000000b8d00 T rust_eh_personality
00000000000dd2d0 T _ZN4core9panicking9panic_old17h9124249c965722d8E
$ nm ./build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/debug/deps/libstd-1c137a8ce7ad3430.so | rg 'eh_person|panic_old'
0000000000529d90 d DW.ref.rust_eh_personality
000000000018bc80 t rust_eh_personality
0000000000249570 t _ZN4core9panicking9panic_old17hd5ed84fef9dfdb68E
