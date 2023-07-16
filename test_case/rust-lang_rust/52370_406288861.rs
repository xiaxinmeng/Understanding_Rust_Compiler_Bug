
➜  proc_macro_limit git:(master) RUST_BACKTRACE=1 cargo build
   Compiling proc_macro_limit v0.1.0 (file:///home/oliver/Projects/rust/proc_macro_limit)
warning: this feature has been stable since 1.29.0. Attribute no longer needed: proc_macro_limit                                                                
 --> src/lib.rs:1:12
  |
1 | #![feature(proc_macro)]
  |            ^^^^^^^^^^
  |
  = note: #[warn(stable_features)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 1.25s                                                                                                   
➜  proc_macro_limit git:(master) rustc --version
rustc 1.29.0-dev
