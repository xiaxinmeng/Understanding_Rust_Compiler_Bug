
error[E0404]: expected trait, found struct `A`                     
 --> src/lib.rs:3:11                                               
  |                                                                
3 | fn f() -> A + 'static {                                        
  |           ^ not a trait                                        
                                                                   
warning: trait objects without an explicit `dyn` are deprecated    
 --> src/lib.rs:3:11                                               
  |                                                                
3 | fn f() -> A + 'static {                                        
  |           ^^^^^^^^^^^ help: use `dyn`: `dyn A + 'static`       
  |                                                                
  = note: `#[warn(bare_trait_objects)]` on by default              
                                                                   
error: aborting due to previous error; 1 warning emitted           
                                                                   
For more information about this error, try `rustc --explain E0404`.
