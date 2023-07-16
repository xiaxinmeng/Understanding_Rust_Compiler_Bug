bash

  Finished test [unoptimized + debuginfo] target(s) in 3m 55s
     Running target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f
==14228==MemorySanitizer: failed to intercept '__isoc99_printf'
'==14228==MemorySanitizer: failed to intercept '__isoc99_sprintf'
'==14228==MemorySanitizer: failed to intercept '__isoc99_snprintf'
'==14228==MemorySanitizer: failed to intercept '__isoc99_fprintf'
'==14228==MemorySanitizer: failed to intercept '__isoc99_vprintf'
'==14228==MemorySanitizer: failed to intercept '__isoc99_vsprintf'
'==14228==MemorySanitizer: failed to intercept '__isoc99_vsnprintf'
'==14228==MemorySanitizer: failed to intercept '__isoc99_vfprintf'
'==14228==MemorySanitizer: failed to intercept 'crypt'
'==14228==MemorySanitizer: failed to intercept 'crypt_r'
'==14228==Installed the sigaction for signal 11
==14228==Installed the sigaction for signal 7
==14228==Installed the sigaction for signal 8
__msan_init 0x559a9a5aa740
app-1: 0 - ffffffffff
shadow-2: 10000000000 - fffffffffff
invalid: 100000000000 - 10ffffffffff
origin-2: 110000000000 - 1fffffffffff
shadow-3: 200000000000 - 2fffffffffff
origin-3: 300000000000 - 3fffffffffff
invalid: 400000000000 - 4fffffffffff
shadow-1: 500000000000 - 50ffffffffff
app-2: 510000000000 - 5fffffffffff
origin-1: 600000000000 - 60ffffffffff
invalid: 610000000000 - 6fffffffffff
app-3: 700000000000 - 7fffffffffff
MemorySanitizer init done
Uninitialized bytes in __interceptor_memchr at offset 0 inside [0x701000000000, 4)
Shadow map of [0x201000000000, 0x201000000004), 4 bytes:
0x201000000000: ffffffff ........ ........ ........

==14228==WARNING: MemorySanitizer: use-of-uninitialized-value
    #0 0x559a9a65e7ba  (/home/crb002/github/coreutils/target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f+0xcb7ba)
    #1 0x559a9a669ad5  (/home/crb002/github/coreutils/target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f+0xd6ad5)
    #2 0x559a9a60715b  (/home/crb002/github/coreutils/target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f+0x7415b)
    #3 0x559a9a606ac1  (/home/crb002/github/coreutils/target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f+0x73ac1)
    #4 0x7f3383b7bd09  (/lib/x86_64-linux-gnu/libc.so.6+0x26d09)
    #5 0x559a9a5aa169  (/home/crb002/github/coreutils/target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f+0x17169)

SUMMARY: MemorySanitizer: use-of-uninitialized-value (/home/crb002/github/coreutils/target/x86_64-unknown-linux-gnu/debug/deps/coreutils-641830cd6c0f495f+0xcb7ba)
Exiting
error: test failed, to rerun pass '--bin coreutils'

