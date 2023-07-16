
error: unreachable pattern                                                                                                                                                                                         
  --> /home/mark/nobackup/rust3/src/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs:51:15                                                                                                                  
   |                                                                                                                                                                                                               
LL |     while let PartiallyInhabitedVariants::Struct { x, .. } = partially_inhabited_variant() {                                                                                                                  
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                                                                                                                    
   |                                                                                                                                                                                                               
note: the lint level is defined here                                                                                                                                                                               
  --> /home/mark/nobackup/rust3/src/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs:3:9                                                                                                                    
   |                                                                                                                                                                                                               
LL | #![deny(unreachable_patterns)]                                                                                                                                                                                
   |         ^^^^^^^^^^^^^^^^^^^^                                                                                                                                                                                  
                                                                                                                                                                                                                   
error: aborting due to previous error 
