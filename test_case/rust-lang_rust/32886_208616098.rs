
$ readelf -d ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc | grep RUNPATH
 0x000000000000001d (RUNPATH)            Library runpath: [$ORIGIN/deps:$ORIGIN/../../../stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib]
