
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
    --> /home/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_session-633.0.0/config.rs:2845:25
     |
2845 |             if !matches.opt_present(opt.name) {
     |                         ^^^^^^^^^^^
     |
     = note: for more information, see https://github.com/rust-lang/rust/issues/27812
     = help: add `#![feature(rustc_private)]` to the crate attributes to enable
