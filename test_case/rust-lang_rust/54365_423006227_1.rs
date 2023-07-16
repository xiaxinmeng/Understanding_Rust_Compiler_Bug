
$ RUSTFLAGS="-Z sanitizer=memory" cargo +nightly run --target x86_64-unknown-linux-gnu
   Compiling main v0.1.2 (/home/malik/workspace/opensource/fitrs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.01s                                                                                                                             
     Running `target/x86_64-unknown-linux-gnu/debug/main`
==7765==WARNING: MemorySanitizer: use-of-uninitialized-value
    #0 0x555e786e526e  (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0xb26e)
    #1 0x555e786e3d63  (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0x9d63)
    #2 0x555e786e4fb5  (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0xafb5)
    #3 0x555e7874a192  (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0x70192)
    #4 0x555e78756c19  (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0x7cc19)
    #5 0x555e78741235  (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0x67235)
    #6 0x555e786e4ede  (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0xaede)
    #7 0x555e786e3dd1  (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0x9dd1)
    #8 0x7f9098f9fb96  (/lib/x86_64-linux-gnu/libc.so.6+0x21b96)
    #9 0x555e786e37f9  (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0x97f9)

SUMMARY: MemorySanitizer: use-of-uninitialized-value (/home/malik/workspace/opensource/fitrs/target/x86_64-unknown-linux-gnu/debug/main+0xb26e) 
Exiting
