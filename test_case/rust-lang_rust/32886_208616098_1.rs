
$ readelf -d ./x86_64-unknown-linux-gnu/stage1/bin/rustc | grep RUNPATH
 0x000000000000001d (RUNPATH)            Library runpath: [$ORIGIN/../lib:/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib]
