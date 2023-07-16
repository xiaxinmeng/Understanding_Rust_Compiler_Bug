
% rustc -V
rustc 1.20.0-nightly (9475ae477 2017-07-11)
% cargo -V
cargo 0.21.0-nightly (eb6cf012a 2017-07-02)
% rustup -V
rustup 1.5.0
% env | grep LD_
[no output]
% cp /usr/lib/libSegFault.so libc.so.6
% cargo clean && env LD_DEBUG=libs cargo build
[...]
rustc: relocation error: libc.so.6: symbol getenv, version GLIBC_2.2.5 not defined in file libc.so.6 with link time reference
error: Could not compile `byteorder`.
[...]
