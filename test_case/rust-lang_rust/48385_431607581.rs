
zmd@ReflectiveCoherence:~/Code/Misc$ cat 48385.rs 
#![feature(nll)]
#![deny(elided_lifetimes_in_paths)]

fn main() {
    format!("foo {}", 22);
}
zmd@ReflectiveCoherence:~/Code/Misc$ rustc +nightly --version
rustc 1.31.0-nightly (e7f5d4805 2018-10-18)
zmd@ReflectiveCoherence:~/Code/Misc$ rustc +nightly 48385.rs 
zmd@ReflectiveCoherence:~/Code/Misc$ echo $?
0
