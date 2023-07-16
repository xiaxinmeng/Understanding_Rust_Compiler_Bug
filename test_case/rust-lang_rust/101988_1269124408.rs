plain
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
[RUSTC-TIMING] rustc_driver test:false 24.042
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
[RUSTC-TIMING] rustc_smir test:false 0.069
error: linking with `x86_64-illumos-gcc` failed: exit status: 1
  |
  = note: "x86_64-illumos-gcc" "/tmp/rustcbrXml3/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-illumos/release/deps/rustc_main-4522ae1117a6739c.rustc_main.52696059-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-illumos/release/deps/rustc_main-4522ae1117a6739c.rustc_main.52696059-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-illumos/release/deps/rustc_main-4522ae1117a6739c.rustc_main.52696059-cgu.2.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-illumos/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-illumos/release/build/psm-3995413cfd626061/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-illumos/release/build/rustc_llvm-31d84931ce092629/out" "-L" "/checkout/obj/build/x86_64-unknown-illumos/llvm/build/lib" "-L" "/opt/illumos/x86_64/lib/gcc/x86_64-pc-solaris2.10/8.4.0/../../../../x86_64-pc-solaris2.10/lib/amd64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-illumos/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-illumos/release/deps" "-lrustc_driver-fe20e476bdad3797" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-illumos/lib" "-lstd-dbca37d6996e03c5" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-illumos/lib/libcompiler_builtins-5843a5198c157050.rlib" "-Wl,-Bdynamic" "-lsendfile" "-llgrp" "-lsocket" "-lposix4" "-lpthread" "-lresolv" "-lnsl" "-lumem" "-lgcc_s" "-lm" "-lrt" "-lpthread" "-lsendfile" "-llgrp" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-illumos/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-illumos/release/deps/rustc_main-4522ae1117a6739c" "-Wl,--gc-sections" "-no-pie" "-Wl,-O1" "-nodefaultlibs" "-Wl,-z,origin" "-Wl,-rpath,$ORIGIN/../lib"
  = note: /opt/illumos/x86_64/lib/gcc/x86_64-pc-solaris2.10/8.4.0/../../../../x86_64-pc-solaris2.10/bin/ld: /opt/illumos/x86_64/sysroot/usr/lib/amd64/crt1.o: undefined reference to symbol 'exit@@SUNW_0.7'
          /opt/illumos/x86_64/sysroot/lib/amd64/libc.so.1: error adding symbols: DSO missing from command line
          collect2: error: ld returned 1 exit status
          
  = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
[RUSTC-TIMING] rustc_main test:false 0.195
error: could not compile `rustc-main` due to previous error
Build completed unsuccessfully in 0:13:59
