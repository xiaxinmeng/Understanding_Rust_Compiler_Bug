plain
........................................................................................ 352/1200
.............................ii......................................................... 440/1200
........................i....i.......................................................... 528/1200
......i......................ii......................................................... 616/1200
.................................................................................F....F. 704/1200
........................................................................................ 880/1200
................................................................................iii..... 968/1200
........................................................................................ 1056/1200
...........................................................iiiiii....................... 1144/1200
...........................................................iiiiii....................... 1144/1200
........................i...............................
failures:

---- src/net/tcp.rs - net::tcp::TcpStream::quickack (line 534) stdout ----
error[E0658]: use of unstable library feature 'tcp_quickack'
  |
  |
8 | stream.set_quickack(true).expect("set_quickack call failed");
  |
  = note: see issue #96256 <https://github.com/rust-lang/rust/issues/96256> for more information
  = note: see issue #96256 <https://github.com/rust-lang/rust/issues/96256> for more information
  = help: add `#![feature(tcp_quickack)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'tcp_quickack'
  |
  |
9 | assert_eq!(stream.quickack().unwrap_or(false), true);
  |
  = note: see issue #96256 <https://github.com/rust-lang/rust/issues/96256> for more information
  = note: see issue #96256 <https://github.com/rust-lang/rust/issues/96256> for more information
  = help: add `#![feature(tcp_quickack)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/net/tcp.rs - net::tcp::TcpStream::set_quickack (line 516) stdout ----
error[E0658]: use of unstable library feature 'tcp_quickack'
  |
  |
8 | stream.set_quickack(true).expect("set_quickack call failed");
  |
  = note: see issue #96256 <https://github.com/rust-lang/rust/issues/96256> for more information
  = note: see issue #96256 <https://github.com/rust-lang/rust/issues/96256> for more information
  = help: add `#![feature(tcp_quickack)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
