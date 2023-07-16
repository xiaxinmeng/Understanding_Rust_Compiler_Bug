
 cargo test                                                                       (~/src/textwrap)
   Compiling tinyvec_macros v0.1.0                                                                          
   Compiling pulldown-cmark v0.1.2                                                                          
   Compiling bitflags v0.9.1                                                                                
   Compiling quote v0.3.15                                                                                  
error[E0658]: using the `?` macro Kleene operator for "at most one" repetition is unstable (see issue #48075)
 --> /home/mg/.cargo/registry/src/github.com-1ecc6299db9ec823/tinyvec_macros-0.1.0/src/lib.rs:9:106         
  |                                                                                                         
9 |         $v:vis fn $fname:ident ($seif:ident : $seifty:ty $(,$argname:ident : $argtype:ty)*) $(-> $ret:ty)? ;
  |                                                                                                          ^
