
...
 Documenting std v0.0.0 (/home/tshepang/rust/library/std)          
error: unresolved link to `crate::os::unix::ffi::OsStrExt`                                                                                   
   --> library/std/src/ffi/mod.rs:134:22
    |                                                                                                                                        
134 | //! [unix.OsStrExt]: crate::os::unix::ffi::OsStrExt                                                                                    
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unresolved link
    |                                                                                                                                        
    = note: `-D broken-intra-doc-links` implied by `-D warnings`      
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
...