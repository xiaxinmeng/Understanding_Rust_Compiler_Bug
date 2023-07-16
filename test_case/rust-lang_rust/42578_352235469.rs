rust
error: `<` is interpreted as a start of generic arguments for `u16`, not a comparison                                          
 --> /tmp/angle.rs:4:29                                                                                                        
  |                                                                                                                            
4 |     println!("{}", x as u16 << 4);                                                                                         
  |                    -------- ^^ - interpreted as generic arguments                                                          
  |                    |        |                                                                                              
  |                    |        not interpreted as comparison                                                                  
  |                    help: try comparing the casted value: `(x as u16)`       
