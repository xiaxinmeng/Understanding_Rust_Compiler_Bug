
error[E0308]: mismatched types                                                       
   --> src/tools/compiletest/src/main.rs:519:22                 
    |                                                                                                            
519 |         run_ignored: config.run_ignored,                                           
    |                      ^^^^^^^^^^^^^^^^^^ expected enum `test::RunIgnored`, found bool
    |                                                                                                             
    = note: expected type `test::RunIgnored`                                                      
               found type `bool`
