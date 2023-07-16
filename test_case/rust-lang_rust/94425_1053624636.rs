plain
[RUSTC-TIMING] object test:false 4.349
[RUSTC-TIMING] gimli test:false 4.394
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error[E0599]: no function or associated item named `bind_with_backlog` found for struct `sys::wasm::net::TcpListener` in the current scope
    |
    |
742 |         super::each_addr(addr, move |a| net_imp::TcpListener::bind_with_backlog(a, backlog))
    |                                                               ^^^^^^^^^^^^^^^^^ function or associated item not found in `sys::wasm::net::TcpListener`
    |
   ::: library/std/src/sys/wasm/../unsupported/net.rs:118:1
    |
118 | pub struct TcpListener(!);
    | -------------------------- function or associated item `bind_with_backlog` not found for this
For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] std test:false 1.393
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
