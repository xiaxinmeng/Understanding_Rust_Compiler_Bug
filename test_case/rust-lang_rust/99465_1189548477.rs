plain
    Checking libc v0.2.126
    Checking xshell v0.2.2
    Checking parser v0.0.0 (/checkout/src/tools/rust-analyzer/crates/parser)
    Checking lock_api v0.4.7
    Checking sourcegen v0.0.0 (/checkout/src/tools/rust-analyzer/crates/sourcegen)
    Checking ena v0.14.0
    Checking tracing-log v0.1.3
    Checking crossbeam-channel v0.5.5
    Checking xflags v0.2.4
---
    Checking regex v1.5.6
    Checking pulldown-cmark v0.9.1
    Checking text-edit v0.0.0 (/checkout/src/tools/rust-analyzer/crates/text-edit)
    Checking crossbeam-deque v0.8.1
    Checking xtask v0.1.0 (/checkout/src/tools/rust-analyzer/xtask)
    Checking countme v3.0.1
    Checking profile v0.0.0 (/checkout/src/tools/rust-analyzer/crates/profile)
    Checking rowan v0.15.5
    Checking idna v0.2.3
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
