text
error[E0308]: mismatched types                                                                                                                                                                                                                                               
 --> src/lib.rs:3:10                                                                                                                                                                                                                              
  |                                                                                                                                                                                                                                                                          
3 | #[derive(CustomMacro)]
4 | #[custom = "-1 as i32"]
  |            ^^^^^^^^^^^ expected `usize`, found `i32`                                                                                                                                                                                                                          
  |                                                                                                                                                                                                                                                                          
  = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info) 
