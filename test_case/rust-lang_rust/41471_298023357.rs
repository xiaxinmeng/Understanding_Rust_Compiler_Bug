
nagisa@debian:~/encoding_rs-master$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading cfg-if v0.1.0
   Compiling cfg-if v0.1.0
   Compiling encoding_rs v0.6.2 (file:///home/nagisa/encoding_rs-master)
    Finished dev [unoptimized + debuginfo] target(s) in 533.65 secs
nagisa@debian:~/encoding_rs-master$ uname -a
Linux debian 3.16.0-4-4kc-malta #1 Debian 3.16.39-1 (2016-12-30) mips GNU/Linux
nagisa@debian:~/encoding_rs-master$ rustc --version
rustc 1.18.0-nightly (94e884b63 2017-04-27)
nagisa@debian:~/encoding_rs-master$ cargo +stable build
   Compiling cfg-if v0.1.0
$<2>error[E0514]$<2>$<2>: found crate `std` compiled by an incompatible version of rustc$<2>
  $<2>$<2>|$<2>
  $<2>$<2>= $<2>$<2>help$<2>: please recompile that crate using this compiler (rustc 1.17.0 (56124baa9 2017-04-24))$<2>
  $<2>$<2>= $<2>$<2>note$<2>: crate `std` path #1: /home/nagisa/.rustup/toolchains/stable-mips-unknown-linux-gnu/lib/rustlib/mips-unknown-linux-gnu/lib/libstd_unicode-cfbd6648f7db2ee5.rlib compiled by ""$<2>
  $<2>$<2>= $<2>$<2>note$<2>: crate `std` path #2: /home/nagisa/.rustup/toolchains/stable-mips-unknown-linux-gnu/lib/rustlib/mips-unknown-linux-gnu/lib/libstd-f4594d3e53dcb114.rlib compiled by ""$<2>
  $<2>$<2>= $<2>$<2>note$<2>: crate `std` path #3: /home/nagisa/.rustup/toolchains/stable-mips-unknown-linux-gnu/lib/rustlib/mips-unknown-linux-gnu/lib/libstd-f4594d3e53dcb114.so compiled by ""$<2>

$<2>error$<2>$<2>: aborting due to previous error$<2>

error: Could not compile `cfg-if`.

To learn more, run the command again with --verbose.
