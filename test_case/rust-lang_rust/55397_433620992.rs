
jchase@atlas foo $ cat Cargo.toml 
[package]
name = "foo"
version = "0.1.0"
authors = ["Josh Robson Chase <josh@robsonchase.com>"]

# [lib]
# crate-type = ["proc-macro"]

[dependencies]
syn = "0.15.14"
quote = "0.6.8"
jchase@atlas foo $ cargo build
   Compiling foo v0.1.0 (/data/home/jchase/src/github.com/jrobsonchase/proc_macro_problem/foo)                                                                                                 
error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type                                                                                                 
 --> src/lib.rs:8:1                                                                                                                                                                            
  |                                                                                                                                                                                            
8 | #[proc_macro]                                                                                                                                                                              
  | ^^^^^^^^^^^^^                                                                                                                                                                              
                                                                                                                                                                                               
error: aborting due to previous error                                                                                                                                                          
                                                                                                                                                                                               
error: Could not compile `foo`.                                                                                                                                                                

To learn more, run the command again with --verbose.
