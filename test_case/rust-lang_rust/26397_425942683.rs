
$ cat 26397.rs 
#![feature(asm)]

fn main() {
    unsafe { asm!("nop" : : : "rcx,r11"); }
}
$ rustc +nightly -vV
rustc 1.30.0-nightly (bb0896af1 2018-09-29)
binary: rustc
commit-hash: bb0896af11bb6b79051d4b794c70f78cd45d080f
commit-date: 2018-09-29
host: x86_64-unknown-linux-gnu
release: 1.30.0-nightly
LLVM version: 8.0
$ rustc +nightly 26397.rs
error[E0668]: malformed inline assembly
 --> 26397.rs:4:14
  |
4 |     unsafe { asm!("nop" : : : "rcx,r11"); }
  |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0668`.
