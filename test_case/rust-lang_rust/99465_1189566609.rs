plain
    Checking lock_api v0.4.7
    Checking always-assert v0.1.2
    Checking ena v0.14.0
    Checking tracing-log v0.1.3
    Checking sourcegen v0.0.0 (/checkout/src/tools/rust-analyzer/crates/sourcegen)
    Checking xflags v0.2.4
    Checking flate2 v1.0.24
    Checking parking_lot_core v0.9.3
    Checking stdx v0.0.0 (/checkout/src/tools/rust-analyzer/crates/stdx)
---
    Checking dashmap v5.3.4
    Checking parking_lot v0.12.1
    Checking crossbeam-deque v0.8.1
    Checking perf-event v0.4.7
    Checking xtask v0.1.0 (/checkout/src/tools/rust-analyzer/xtask)
    Checking regex v1.5.6
    Checking pulldown-cmark v0.9.1
    Checking text-edit v0.0.0 (/checkout/src/tools/rust-analyzer/crates/text-edit)
    Checking notify v5.0.0-pre.15
---
    |
282 |     type Ident = IdentId;
    |     ^^^^^^^^^^^^^^^^^^^^^ not a member of trait `server::Types`

error[E0437]: type `Literal` is not a member of trait `server::Types`
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:283:5
283 |     type Literal = Literal;
    |     ^^^^^^^^^^^^^^^^^^^^^^^ not a member of trait `server::Types`

error[E0404]: expected trait, found struct `server::Ident`
---
    |
3   | use super::*;
    |     ^^^^^^^^

error[E0603]: struct `Literal` is private
   --> crates/proc-macro-srv/src/abis/abi_sysroot/rustc_server.rs:465:14
465 | impl server::Literal for Rustc {
    |              ^^^^^^^ private struct
    |
    |
note: the struct `Literal` is defined here
   --> /checkout/library/proc_macro/src/bridge/server.rs:3:5
3   | use super::*;
    |     ^^^^^^^^

Some errors have detailed explanations: E0404, E0437, E0603.
