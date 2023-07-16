
  = note: /usr/bin/ld: /usr/lib/gcc/x86_64-redhat-linux/9/../../../../lib64/Scrt1.o: in function `_start':
          (.text+0x16): undefined reference to `__libc_csu_fini'
          /usr/bin/ld: (.text+0x1d): undefined reference to `__libc_csu_init'
          /usr/bin/ld: (.text+0x2a): undefined reference to `__libc_start_main'
          /usr/bin/ld: /home/pnkfelix/.rustup/toolchains/nightly-2019-03-29-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore\
-ed8b00cc009d913d.rlib(core-ed8b00cc009d913d.core.95lhmtlm-cgu.0.rcgu.o): in function `equal<u8>':
          /rustc/237bf3244fffef501cf37d4bda00e1fce3fcfb46//src/libcore/slice/mod.rs:5074: undefined reference to `memcmp'
