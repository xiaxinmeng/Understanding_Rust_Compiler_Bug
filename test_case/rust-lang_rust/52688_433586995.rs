console
$ cargo new --bin rust_52688_repro
     Created binary (application) `rust_52688_repro` project
$ cd rust_52688_repro/
$ echo 'typenum = "=1.10.0"' >> Cargo.toml 
$ cargo doc
    Updating crates.io index
   Compiling typenum v1.10.0                                                                                                      
 Documenting typenum v1.10.0                                                                                                      
error: internal compiler error: librustc/traits/structural_impls.rs:178: impossible case reached                                  
                                                                                                                                  
thread '<unnamed>' panicked at 'Box<Any>', librustc_errors/lib.rs:587:9                                                           
note: Run with `RUST_BACKTRACE=1` for a backtrace.                                                                                
                                                                                                                                  
note: the compiler unexpectedly panicked. this is a bug.                                                                          
                                                                                                                                  
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports                 
                                                                                                                                  
note: rustc 1.30.0 (da5f414c2 2018-10-24) running on x86_64-apple-darwin                                                          
                                                                                                                                  
error: Could not document `typenum`.                                                                                              

Caused by:
  process didn't exit successfully: `rustdoc --crate-name typenum /Users/willglynn/.cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.10.0/src/lib.rs --cap-lints allow --color always -o /Users/delta407/rust/rust_52688_repro/target/doc -L dependency=/Users/willglynn/rust/rust_52688_repro/target/debug/deps` (exit code: 1)
