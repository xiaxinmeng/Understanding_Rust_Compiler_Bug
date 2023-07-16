
   Compiling recrypt v0.5.0 (/workspace/recrypt-rs)                                                                                  
error: internal compiler error: src/librustc/traits/codegen/mod.rs:68: Encountered error `Unimplemented` selecting `Binder(<core::char::DecodeUtf16<<std::vec::Vec<u16> as core::iter::IntoIterator>::IntoIter> as arbitrary::traits::Arbitrary>)` during codegen
                                                                                                                                                 
thread 'main' panicked at 'Box<Any>', src/librustc_errors/lib.rs:600:9                                                                           
note: Run with `RUST_BACKTRACE=1` for a backtrace.                                                                                               
error: aborting due to previous error                                                                                                            
                                                                                                                                                                                                                                                                                                  
note: the compiler unexpectedly panicked. this is a bug.                                                                                         
                                                                                                                                                 
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports                                
                                                                                                                                                 
note: rustc 1.32.0-nightly (f4a421ee3 2018-12-13) running on x86_64-unknown-linux-gnu                                                            
                                                                                                                                                 
note: compiler flags: -C opt-level=2 -C debuginfo=2 -C debug-assertions=on -C relocation-model=dynamic-no-pic -C link-dead-code -C opt-level=0 --crate-type lib
                                                                                                                                                 
note: some of the compiler flags provided by cargo are hidden                                                                                    
                                                                                                                                                 
error: Could not compile `proptest`. 
