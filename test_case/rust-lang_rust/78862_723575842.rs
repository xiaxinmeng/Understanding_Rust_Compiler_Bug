
help: you can convert an `i32` to `usize` and panic if the converted value wouldn't fit                                                                                                                                                                                      
  |                                                                                                                                                                                                                                                                          
4 | #[custom = "{ (-1 as i32).try_into().unwrap() }"]                                                                                                                                                                                                                                
  |   
