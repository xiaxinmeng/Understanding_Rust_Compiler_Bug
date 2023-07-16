
$ rustc +nightly --version
rustc 1.56.0-nightly (492723897 2021-07-29)
$ cat > temp.rs
fn main() {}
$ rustc +nightly --edition 2021 -Z unstable-options -C lto temp.rs
$
