console
% rustup default nightly-2018-09-14
info: using existing install for 'nightly-2018-09-14-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-2018-09-14-x86_64-unknown-linux-gnu'

  nightly-2018-09-14-x86_64-unknown-linux-gnu unchanged - rustc 1.30.0-nightly (90d36fb59 2018-09-13)

% cargo new issue-54654
     Created binary (application) `issue-54654` project
% cd issue-54654/
% echo 'interpolate = "0.2.1"' >> Cargo.toml
% cat Cargo.toml
[package]
name = "issue-54654"
version = "0.1.0"
authors = ["Felix S. Klock II <pnkfelix@pnkfx.org>"]
edition = "2018"

[dependencies]
interpolate = "0.2.1"
% echo '#[macro_use] extern crate interpolate; fn main() { }' > src/main.rs
% cat src/main.rs
#[macro_use] extern crate interpolate; fn main() { }
% cargo build
    Updating crates.io index
   Compiling proc-macro2 v0.4.20
   Compiling unicode-xid v0.1.0
   Compiling quote v0.6.8
   Compiling interpolate v0.2.1
   Compiling issue-54654 v0.1.0 (file:///tmp/issue-54654)
warning: unused `#[macro_use]` import
 --> src/main.rs:1:1
  |
1 | #[macro_use] extern crate interpolate; fn main() { }
  | ^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 2.99s
%
