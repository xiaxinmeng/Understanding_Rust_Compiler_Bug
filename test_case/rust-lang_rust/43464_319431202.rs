rust
   Compiling playground v0.0.1 (file:///playground)
error[E0277]: the trait bound `std::net::IpAddr: std::convert::From<({integer}, {integer}, {integer}, {integer})>` is not satisfied
 --> src/main.rs:2:42
  |
2 |     let _x: std::net::IpAddr = (0,0,0,0).into();
  |                                          ^^^^ the trait `std::convert::From<({integer}, {integer}, {integer}, {integer})>` is not implemented for `std::net::IpAddr`
  |
  = help: the following implementations were found:
            <std::net::IpAddr as std::convert::From<[u16; 8]>>
            <std::net::IpAddr as std::convert::From<[u8; 4]>>
            <std::net::IpAddr as std::convert::From<std::net::Ipv6Addr>>
            <std::net::IpAddr as std::convert::From<std::net::Ipv4Addr>>
            <std::net::IpAddr as std::convert::From<[u8; 16]>>
  = note: required because of the requirements on the impl of `std::convert::Into<std::net::IpAddr>` for `({integer}, {integer}, {integer}, {integer})`
