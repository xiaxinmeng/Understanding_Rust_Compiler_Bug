
[01:13:00] error[E0599]: no variant named `GlobalAsm` found for type `rustc::hir::def::Def` in the current scope
[01:13:00]    --> tools/clippy/clippy_lints/src/utils/mod.rs:942:9
[01:13:00]     |
[01:13:00] 942 |         Def::GlobalAsm(id) => Some(id),
[01:13:00]     |         ^^^^^^^^^^^^^^^^^^ variant not found in `rustc::hir::def::Def`
