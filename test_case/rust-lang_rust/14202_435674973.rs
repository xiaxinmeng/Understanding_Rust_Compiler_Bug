
#![feature(rustc_attrs)]                                                        
...
#[rustc_mir(borrowck_graphviz_postflow="/tmp/suffix.dot")]                      
fn main() {
