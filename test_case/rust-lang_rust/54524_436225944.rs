
error: `[grouped_by]` cannot be resolved, ignoring it...                                                                                                        
   --> diesel/src/associations/mod.rs:204:6                                                                                                                     
    |                                                                                                                                                           
204 | //! [`grouped_by`] is called on a `Vec<Child>` with a `&[Parent]`.                                                                                        
    |      ^^^^^^^^^^^^ cannot be resolved, ignoring                                                                                                            
    |                                                                                                                                                           
note: lint level defined here                                                                                                                                   
   --> diesel/src/lib.rs:131:9                                                                                                                                  
    |                                                                                                                                                           
131 | #![deny(warnings, missing_debug_implementations, missing_copy_implementations, missing_docs)]                                                             
    |         ^^^^^^^^                                                                                                                                          
    = note: #[deny(intra_doc_link_resolution_failure)] implied by #[deny(warnings)]                                                                             
    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
