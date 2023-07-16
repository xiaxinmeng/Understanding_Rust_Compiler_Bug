zsh
 % rustup show
Default host: x86_64-unknown-linux-gnu

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
nightly-2017-12-21-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu (default)

active toolchain
----------------

nightly-x86_64-unknown-linux-gnu (default)
rustc 1.26.0-nightly (2789b067d 2018-03-06)

 % cat src/main.rs
fn main() {
    let a = [1, 2, 3];
    let index = 10;
    let element = a[index];
    println!("element is {}", element);
}
 % rustc src/main.rs -C debuginfo=2
 % rustc src/main.rs -C debuginfo=1
warning: this expression will panic at run-time
 --> src/main.rs:4:19
  |
4 |     let element = a[index];
  |                   ^^^^^^^^ index out of bounds: the len is 3 but the index is 10

