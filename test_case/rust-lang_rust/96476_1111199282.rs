plain
[RUSTC-TIMING] kqueue test:false 1.018
   Compiling notify v5.0.0-pre.14
[RUSTC-TIMING] unicode_normalization test:false 1.749
   Compiling dashmap v5.2.0
error[E0004]: non-exhaustive patterns: `Open`, `CloseWrite` and `Close` not covered
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/notify-5.0.0-pre.14/src/kqueue.rs:152:39
152 |                     let event = match data {
152 |                     let event = match data {
    |                                       ^^^^ patterns `Open`, `CloseWrite` and `Close` not covered
    |
note: `kqueue::Vnode` defined here
    |
    |
60  | / pub enum Vnode {
61  | |     /// The file was deleted
62  | |     Delete,
...   |
86  | |     Open,
    | |     ^^^^ not covered
...   |
...   |
89  | |     CloseWrite,
    | |     ^^^^^^^^^^ not covered
...   |
92  | |     Close,
    | |     ^^^^^ not covered
93  | | }
    | |_-
    = note: the matched value is of type `kqueue::Vnode`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
265 ~                         }
266 + 
266 + 
267 +                         // Access to the file was revoked via revoke(2) or the underlying file system was unmounted.
268 +                         Open | CloseWrite | Close => todo!()

[RUSTC-TIMING] text_edit test:false 1.359
   Compiling idna v0.2.3
For more information about this error, try `rustc --explain E0004`.
---
[RUSTC-TIMING] object test:false 5.077
[RUSTC-TIMING] pulldown_cmark test:false 3.483
[RUSTC-TIMING] syn test:false 6.342
[RUSTC-TIMING] syn test:false 6.204
thread 'main' panicked at 'rust-analyzer always builds', src/bootstrap/dist.rs:1080:14
[TIMING] tool::RustAnalyzer { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-netbsd, extra_features: [] } -- 0.000
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:19:34
