console
$ rustup update
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.51.0 (2fd73fabe 2021-03-23)

info: cleaning up downloads & tmp directories
$ cargo clean && cargo check --lib && git apply add-dyn.patch && cargo check --lib ; git apply -R add-dyn.patch
   Compiling proc-macro2 v1.0.26
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.68
   Compiling serde_derive v1.0.125
   Compiling serde v1.0.125
   Compiling quote v1.0.9
    Checking incremental-ice v0.0.0 (/home/arthur/Documents/0d19db457b7c0c397da6229db828efdd-edf9417eb91da10bd30ccb7da970ffc349aaac6b)
warning: trait objects without an explicit `dyn` are deprecated
 --> config.rs:8:25
  |
8 |     pub instrument: Box<Write>,
  |                         ^^^^^ help: use `dyn`: `dyn Write`
  |
  = note: `#[warn(bare_trait_objects)]` on by default

warning: trait objects without an explicit `dyn` are deprecated
 --> config.rs:8:25
  |
8 |     pub instrument: Box<Write>,
  |                         ^^^^^ help: use `dyn`: `dyn Write`

warning: 2 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 10.89s
    Checking incremental-ice v0.0.0 (/home/arthur/Documents/0d19db457b7c0c397da6229db828efdd-edf9417eb91da10bd30ccb7da970ffc349aaac6b)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s

