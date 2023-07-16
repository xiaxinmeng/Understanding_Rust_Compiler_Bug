
error[E0460]: found possibly newer version of crate `std` which `synstructure` depends on                                                    
 --> src/librustc_macros/src/lib.rs:4:5                                                                                                      
  |                                                                                                                                          
4 | use synstructure::decl_derive;                                                                                                           
  |     ^^^^^^^^^^^^                                                                                                                         
  |                                                                                                                                                                                                                                                                                       
  = note: perhaps that crate needs to be recompiled?                                                                                                                                                                                                                                      
  = note: the following crate versions were found:                                                                                                                                                                                                                                        
          crate `std`: /home/tshepang/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-95e90a1be2e66993.rlib                                                                                                                            
          crate `std`: /home/tshepang/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-95e90a1be2e66993.so                                                                            
          crate `synstructure`: /home/tshepang/rust/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/libsynstructure-7a0d29a374b01f60.rmeta
