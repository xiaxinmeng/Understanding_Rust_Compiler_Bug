
$ nm /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rls | grep ' malloc'
                 U malloc@@GLIBC_2.2.5
0000000001820a98 d malloc_impl

$ nm /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc | grep " malloc"
00000000000069a0 T malloc
