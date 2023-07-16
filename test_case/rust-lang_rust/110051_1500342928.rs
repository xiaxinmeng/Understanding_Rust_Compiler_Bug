
Building tool jsondoclint (stage0)
   Compiling libc v0.2.140
   Compiling io-lifetimes v1.0.3
   Compiling rustix v0.36.5
   Compiling bitflags v1.3.2
   Compiling linux-raw-sys v0.1.4
   Compiling utf8parse v0.2.1
   Compiling anstyle v0.3.5
   Compiling concolor-query v0.3.3
   Compiling concolor-override v1.0.0
   Compiling heck v0.4.0
   Compiling syn v2.0.8
   Compiling serde_json v1.0.85
   Compiling anyhow v1.0.65
   Compiling anstyle-parse v0.1.1
   Compiling strsim v0.10.0
   Compiling clap_lex v0.4.1
   Compiling rustdoc-json-types v0.1.0 (/home/alona/dev/rust/rust/src/rustdoc-json-types)
error[E0725]: the feature `rustc_private` is not in the list of allowed features
 --> src/rustdoc-json-types/lib.rs:6:12
  |
6 | #![feature(rustc_private)]
  |            ^^^^^^^^^^^^^

error[E0463]: can't find crate for `rustc_hash`
 --> src/rustdoc-json-types/lib.rs:7:1
  |
7 | extern crate rustc_hash;
  | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
  |
  = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`

   Compiling rustc-hash v1.1.0
 