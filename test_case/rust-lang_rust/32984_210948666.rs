
pi@raspberrypi:~/demo $ rustup update
info: syncing channel updates for 'nightly-armv7-unknown-linux-gnueabihf'
info: checking for self-updates
info: rustup is up to date

  nightly-armv7-unknown-linux-gnueabihf unchanged - rustc 1.10.0-nightly (2174bd97c 2016-04-14)

pi@raspberrypi:~/demo $ cat x.rs
#![feature(rustc_private)]
extern crate syntax;

fn main() {
}
pi@raspberrypi:~/demo $ rustc x.rs
x.rs:2:1: 2:21 error: can't find crate for `syntax` [E0463]
x.rs:2 extern crate syntax;
       ^~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
