rust
error: `net::addr::SocketAddrV4::new` is not yet stable as a const fn
   --> src/libstd/net/addr.rs:131:45
    |
131 |             IpAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(a, port)),
    |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add `#![feature(const_socket_new)]` to the crate attributes to enable

error: `net::addr::SocketAddrV6::new` is not yet stable as a const fn
   --> src/libstd/net/addr.rs:132:45
    |
132 |             IpAddr::V6(a) => SocketAddr::V6(SocketAddrV6::new(a, port, 0, 0)),
    |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: add `#![feature(const_socket_new)]` to the crate attributes to enable

error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   --> src/libstd/net/addr.rs:276:28
    |
276 |                 sin_addr: *ip.as_inner(),
    |                            ^^^^^^^^^^^^^

error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   --> src/libstd/net/addr.rs:277:28
    |
277 |                 ..unsafe { mem::zeroed() }
    |                            ^^^^^^^^^^^^^

error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   --> src/libstd/net/addr.rs:369:29
    |
369 |                 sin6_addr: *ip.as_inner(),
    |                             ^^^^^^^^^^^^^

error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   --> src/libstd/net/addr.rs:372:28
    |
372 |                 ..unsafe { mem::zeroed() }
    |                            ^^^^^^^^^^^^^

error: aborting due to 6 previous errors
