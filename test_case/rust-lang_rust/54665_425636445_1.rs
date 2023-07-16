rust
error[E0407]: method `extra` is not a member of trait `salsa::QueryContext`                                                                              
  --> examples/storage_varieties/implementation.rs:30:5                                                                                                  
   |                                                                                                                                                     
30 | /     fn extra(&self) -> u32 {                                                                                                                      
31 | |         0                                                                                                                                         
32 | |     }                                                                                                                                             
   | |_____^ not a member of trait `salsa::QueryContext`                                                                                                 
                                                     
