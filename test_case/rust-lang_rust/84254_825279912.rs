
the libc dependency in backtrace: (with the target_env/os error you mentioned)
    -lc
    -lm
    -lrt
    -lpthread

library/std/src/sys/unix/mod.rs:
    -lsocket
    -lposix4
    -lpthread
    -lresolv
    -lnsl
    -lumem

library/unwind/build.rs:  (my patched copy)
    -lgcc_s
    -lc

vendor/libc/src/unix/mod.rs: (also has the target_env bug)
    -lc
    -lm
    -lrt
    -lpthread

late link args in rustc:
    -lssp
