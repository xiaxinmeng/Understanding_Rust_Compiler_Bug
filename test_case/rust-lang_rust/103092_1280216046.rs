
error: linking with `gcc` failed: exit status: 1                                                                                                                                                         
  |                                                                                                                                                                                                      
  = note: "gcc" "-m64" "-std=c99" "/tmp/rustcsVN1FO/symbols.o" "/tmp/bat/target/release/deps/bat-e4fceccbe6e2b327.bat.7aa865dd-cgu.0.rcgu.o" "-Wl,-z,ignore" "-L" "/tmp/bat/targe
t/release/deps" "-L" "/tmp/bat/target/release/build/libgit2-sys-b78d3b7d704ff41e/out/build" "-L" "/usr/lib/amd64" "-L" "/tmp/bat/target/release/build/onig_sys-159ec9d31ec0a92a/o
ut" "-L" "/home/luqman/.rustup/toolchains/nightly-x86_64-unknown-illumos/lib/rustlib/x86_64-unknown-illumos/lib" "-Wl,-Bstatic" "/tmp/rustcsVN1FO/liblibgit2_sys-e95cd72243699990.rlib" "/tmp/rustcsVN1FO
/libonig_sys-8c63ac69bc862d8b.rlib" "/home/luqman/.rustup/toolchains/nightly-x86_64-unknown-illumos/lib/rustlib/x86_64-unknown-illumos/lib/libcompiler_builtins-5843a5198c157050.rlib" "-Wl,-Bdynamic" "-
lkstat" "-lz" "-lsendfile" "-llgrp" "-lsocket" "-lposix4" "-lpthread" "-lresolv" "-lnsl" "-lumem" "-lgcc_s" "-lm" "-lrt" "-lpthread" "-lsendfile" "-llgrp" "-lc" "-lssp" "-L" "/home/luqman/.rustup/toolc
hains/nightly-x86_64-unknown-illumos/lib/rustlib/x86_64-unknown-illumos/lib" "-o" "/tmp/bat/target/release/deps/bat-e4fceccbe6e2b327" "-nodefaultlibs"                                       
  = note: Undefined                     first referenced                                                                                                                                                 
           symbol                           in file                                                                                                                                                      
          rust_begin_unwind                   /tmp/rustcsVN1FO/symbols.o                                                                                                                                 
          rust_oom                            /tmp/rustcsVN1FO/symbols.o                                                                                                                                 
          ld: fatal: symbol referencing errors. No output written to /tmp/bat/target/release/deps/bat-e4fceccbe6e2b327                                                                       
          collect2: error: ld returned 1 exit status                                                                                                                                                     
                                                                                                                                                                                                         
                                                                                                                                                                                                         
error: could not compile `bat` due to previous error
