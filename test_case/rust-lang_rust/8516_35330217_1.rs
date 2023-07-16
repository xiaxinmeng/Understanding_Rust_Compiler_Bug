
foo.rs:3:1: 6:2 warning: code is never used: `Either`, #[warn(dead_code)] on by default                                        
foo.rs:3 enum Either {                                                                                                         
foo.rs:4     One(()),                                                                                                          
foo.rs:5     Other { left:~str, right:~str}                                                                                    
foo.rs:6 }                                                                                                                     
foo.rs:8:1: 8:31 warning: code is never used: `one`, #[warn(dead_code)] on by default                                          
foo.rs:8 static one : Either = One(());                                                                                        
         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~                                                                                        
error: internal compiler error: unexpected failure                                                                             
This message reflects a bug in the Rust compiler.                                                                              
We would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html                             
note: the compiler hit an unexpected failure path. this is a bug                                                               
Ok(task 'rustc' failed at 'assertion failed: !is_undef(wrapped)', /build/rust-git/src/rust/src/librustc/middle/trans/adt.rs:796
)                                                                                                                              
