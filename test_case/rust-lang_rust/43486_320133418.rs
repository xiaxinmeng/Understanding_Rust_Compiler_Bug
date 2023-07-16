
root@01ae1d1cce84:/workdir# nm /workdir/target/release/deps/libdiesel_codegen-5bbd71c5b82e9c69.so | grep strlcpy
                 U strlcpy
root@01ae1d1cce84:/workdir# nm -D /lib/x86_64-linux-gnu/libc.so.6 | grep strlcpy
root@01ae1d1cce84:/workdir#
