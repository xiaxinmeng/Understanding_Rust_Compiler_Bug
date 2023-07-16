rust
error[E0658]: `match` is not allowed in a `const fn`
   --> src/libstd/net/addr.rs:130:9
    |
130 | /         match ip {
131 | |             IpAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(a, port)),
132 | |             IpAddr::V6(a) => SocketAddr::V6(SocketAddrV6::new(a, port, 0, 0)),
133 | |         }
    | |_________^
    |
    = note: for more information, see https://github.com/rust-lang/rust/issues/49146
    = help: add `#![feature(const_if_match)]` to the crate attributes to enable
