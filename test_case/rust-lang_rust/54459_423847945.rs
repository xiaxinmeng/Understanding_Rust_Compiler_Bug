
$ touch foo.rs
$ export RUST_BACKTRACE=1
$ time rustc +nightly foo.rs
error[E0601]: `main` function not found in crate `foo`                           
  |                                                                              
  = note: consider adding a `main` function to `foo.rs`                          
                                                                                 
error: aborting due to previous error                                            
                                                                                 
For more information about this error, try `rustc --explain E0601`.              
rustc +nightly foo.rs  0.05s user 0.02s system 79% cpu 0.088 total               
$ time rustc +5fd88ccbf4d20c70960c1b0e76fe82325e23a35e foo.rs 
error[E0601]: `main` function not found in crate `foo`                                             
  |                                                                                                
  = note: consider adding a `main` function to `foo.rs`                                            
                                                                                                   
error: aborting due to previous error                                                              
                                                                                                   
For more information about this error, try `rustc --explain E0601`.                                
rustc +5fd88ccbf4d20c70960c1b0e76fe82325e23a35e foo.rs  0.05s user 0.03s system 81% cpu 0.088 total
