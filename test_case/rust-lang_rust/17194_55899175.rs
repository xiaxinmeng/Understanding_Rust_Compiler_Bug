
-bash-4.1# ./install.sh 
install: looking for install programs
install: found mkdir
install: found printf
install: found cut
install: found grep
install: found uname
install: found tr
install: found sed
install: 
install: processing ./install.sh args
install: 
install: CFG_PREFIX           := /usr/local 
install: CFG_LIBDIR           := /usr/local/lib 
install: CFG_MANDIR           := /usr/local/share/man 
install: 
install: validating ./install.sh args
install: 
install: verifying platform can run binaries
/root/rust-nightly-x86_64-unknown-linux-gnu/bin/rustc: /usr/lib64/libstdc++.so.6: version `GLIBCXX_3.4.15' not found (required by /root/rust-nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-4e7c5e5c.so)
install: error: can't execute rustc binary on this platform
-bash-4.1# ldd /root/rust-nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-4e7c5e5c.so
ldd: warning: you do not have execution permission for `/root/rust-nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-4e7c5e5c.so'
/root/rust-nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-4e7c5e5c.so: /usr/lib64/libstdc++.so.6: version `GLIBCXX_3.4.15' not found (required by /root/rust-nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-4e7c5e5c.so)
        linux-vdso.so.1 =>  (0x00007fff0d3fe000)
        librustrt-4e7c5e5c.so => not found
        libpthread.so.0 => /lib64/libpthread.so.0 (0x00007f7c32141000)
        libdl.so.2 => /lib64/libdl.so.2 (0x00007f7c31f3d000)
        libm.so.6 => /lib64/libm.so.6 (0x00007f7c31cb9000)
        libstdc++.so.6 => /usr/lib64/libstdc++.so.6 (0x00007f7c319b2000)
        libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x00007f7c3179c000)
        libc.so.6 => /lib64/libc.so.6 (0x00007f7c31408000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f7c342a5000)
-bash-4.1# cat /etc/redhat-release 
CentOS release 6.5 (Final)
-bash-4.1# strings /usr/lib64/libstdc++.so.6 | grep GLIBC
GLIBCXX_3.4
GLIBCXX_3.4.1
GLIBCXX_3.4.2
GLIBCXX_3.4.3
GLIBCXX_3.4.4
GLIBCXX_3.4.5
GLIBCXX_3.4.6
GLIBCXX_3.4.7
GLIBCXX_3.4.8
GLIBCXX_3.4.9
GLIBCXX_3.4.10
GLIBCXX_3.4.11
GLIBCXX_3.4.12
GLIBCXX_3.4.13
GLIBC_2.2.5
GLIBC_2.3
GLIBC_2.4
GLIBC_2.3.2
GLIBCXX_FORCE_NEW
GLIBCXX_DEBUG_MESSAGE_LENGTH

