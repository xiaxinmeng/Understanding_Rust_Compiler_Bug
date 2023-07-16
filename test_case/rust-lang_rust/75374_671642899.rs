
$ cat ~/.cargo/config
[target.i686-pc-windows-gnu]
rustflags=["-Clink-arg=-fuse-ld=lld"]
[target.x86_64-pc-windows-gnu]
rustflags=["-Clink-arg=-fuse-ld=lld"]
