 console
$ cd $(rustc +stage1 --print sysroot)

$ find -name 'libproc_macro*'
./lib/libproc_macro-09d5bb2a63717cb1.so
./lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-956e91f4391bd306.so
