
albe@myo ~/code/repro [master] % docker build -t repro .
[...]
albe@myo ~/code/repro [master] % docker run --rm -it repro bash
root@c74a4c84b096:/usr/src/repro# rustup default 76a252ea9e7be93a61ffdf33b3533e24a9cf459d
info: default toolchain set to '76a252ea9e7be93a61ffdf33b3533e24a9cf459d'
root@c74a4c84b096:/usr/src/repro# rustc --version
rustc 1.41.0-nightly (76a252ea9 2019-12-09)
root@c74a4c84b096:/usr/src/repro# ./repro-min.sh
ld: target/i686-unknown-linux-gnu/release/librepro.a(std-c3eaafedbae89cb4.std.52ue7rpt-cgu.0.rcgu.o): warning: relocation against `__rust_probestack' in read-only section `.text._ZN3std3sys4unix2fs4copy17h783e034bf468da7dE'
ld: warning: creating a DT_TEXTREL in a shared object
root@c74a4c84b096:/usr/src/repro#
root@c74a4c84b096:/usr/src/repro#
root@c74a4c84b096:/usr/src/repro# rustup default 7de9402b77ded0d8ec9e1c554521b2121449ef2b
info: default toolchain set to '7de9402b77ded0d8ec9e1c554521b2121449ef2b'
root@c74a4c84b096:/usr/src/repro# rustc --version
rustc 1.41.0-nightly (7de9402b7 2019-12-09)
root@c74a4c84b096:/usr/src/repro# ./repro-min.sh
root@c74a4c84b096:/usr/src/repro# echo $?
0
root@c74a4c84b096:/usr/src/repro#
