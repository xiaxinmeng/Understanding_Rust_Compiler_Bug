plain
...............................................................................................i.... 500/1189
i................................................................i.................................. 600/1189
.................................................................................................... 700/1189
.................................................................................................... 800/1189
.........................F......................................................................FF.. 900/1189
.................................................................................................... 1100/1189
...................iiiiii................................................................
failures:


---- src/os/./unix/net/ancillary/ip.rs - os::doc::unix::net::ancillary::ip::IpAncillary::new (line 153) stdout ----
error[E0432]: unresolved import `std::os::unix::net::SocketAncillary`
 --> src/os/./unix/net/ancillary/ip.rs:156:5
6 | use std::os::unix::net::SocketAncillary;
6 | use std::os::unix::net::SocketAncillary;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `SocketAncillary` in `os::unix::net`

error[E0433]: failed to resolve: use of undeclared type `IpAncillary`
 --> src/os/./unix/net/ancillary/ip.rs:158:21
  |
8 | let mut ancillary = IpAncillary::new(&mut ancillary_buffer[..]);
  |
help: consider importing this struct
  |
5 | use std::os::unix::net::IpAncillary;
5 | use std::os::unix::net::IpAncillary;
  |

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
Couldn't compile the test.
---- src/os/./unix/net/udp.rs - os::doc::unix::net::udp::UdpSocket::send_vectored_with_ancillary (line 138) stdout ----
error[E0599]: no method named `add_ipttl` found for struct `IpAncillary` in the current scope
   |
   |
21 |     ancillary.add_ipttl(20);
   |               ^^^^^^^^^ help: there is an associated function with a similar name: `add_ttl`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/os/./unix/net/udp.rs - os::doc::unix::net::udp::UdpSocket::send_vectored_with_ancillary_to (line 178) stdout ----
error[E0599]: no method named `add_ipttl` found for struct `IpAncillary` in the current scope
   |
   |
20 |     ancillary.add_ipttl(20);
   |               ^^^^^^^^^ help: there is an associated function with a similar name: `add_ttl`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/os/./unix/net/ancillary/ip.rs - os::doc::unix::net::ancillary::ip::IpAncillary::new (line 153)
    src/os/./unix/net/udp.rs - os::doc::unix::net::udp::UdpSocket::send_vectored_with_ancillary_to (line 178)

test result: FAILED. 1165 passed; 3 failed; 21 ignored; 0 measured; 0 filtered out; finished in 14.75s


error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:15:42
