plain
Z:\test> rustc +nightly-gnu --version
rustc 1.56.0-nightly (492723897 2021-07-29)
Z:\test> rustc +nightly-msvc --version
rustc 1.56.0-nightly (492723897 2021-07-29)
Z:\test> 'fn main() {}' > temp.rs
Z:\test> rustc +nightly-gnu --edition 2021 -Z unstable-options -C lto temp.rs
Z:\test> rustc +nightly-msvc --edition 2021 -Z unstable-options -C lto temp.rs
Z:\test>
