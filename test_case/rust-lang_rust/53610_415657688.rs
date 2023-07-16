
[RUSTC-TIMING] cargo test:false 127.561
error[E0432]: unresolved import `syntax::codemap`
  --> tools\rls\src\build\rustc.rs:22:13
   |
22 | use syntax::codemap::{FileLoader, RealFileLoader};
   |             ^^^^^^^ Could not find `codemap` in `syntax`
error: cannot find macro `json_internal!` in this scope
   --> tools\rls\src\actions\mod.rs:506:9
    |
506 | /         json!({
507 | |             "watchers": watchers
508 | |         })
    | |__________^
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
