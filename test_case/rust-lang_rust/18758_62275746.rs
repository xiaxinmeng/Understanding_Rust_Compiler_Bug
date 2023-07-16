 rust
 fn next(&'a mut self) -> Option<&'a str>
//                                      ^ It point to this as the problem
//        ^ But this is the actual problem
