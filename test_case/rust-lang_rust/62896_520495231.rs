bash
$ uname -a
Linux rpi2 4.19.65-1-ARCH #1 SMP PREEMPT Fri Aug 9 23:29:04 UTC 2019 armv7l GNU/Linux
$ rustc +stable --version
rustc 1.36.0 (a53f9df32 2019-07-03)
$ rustc +nightly --version
rustc 1.38.0-nightly (00ee1b47f 2019-08-11)

# A trivial hello world program.
$ echo 'fn main() { println!("hello world"); }' > example.rs

# These all work fine.
$ rustc +stable example.rs
$ rustc +stable example.rs -O
$ rustc +nightly example.rs

# This one fails.
$ rustc +nightly example.rs -O
zsh: segmentation fault (core dumped)  rustc +nightly example.rs -O
