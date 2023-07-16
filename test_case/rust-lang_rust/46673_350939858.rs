
$ cat scratch.rs 
fn main() {
    "X".split('X').count();
}
$ RUSTC_BACKTRACE=1 rustc +nightly scratch.rs 
$ echo $?
0
$ ./scratch 
$ rustc +nightly --version --verbose
rustc 1.24.0-nightly (9fe7aa353 2017-12-11)
binary: rustc
commit-hash: 9fe7aa353fac5084d0a44d6a15970310e9be67f4
commit-date: 2017-12-11
host: x86_64-unknown-linux-gnu
release: 1.24.0-nightly
LLVM version: 4.0
