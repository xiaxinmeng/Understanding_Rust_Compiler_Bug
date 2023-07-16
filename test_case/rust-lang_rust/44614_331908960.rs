 rust
#![allow(dead_code)]
use std::fmt::Debug;
struct S(Box<Debug + 'static>);
impl S {
    fn bar<'a>(&'a mut self)->&'a mut Box<Debug + 'a> {
        match self.0 { ref mut x => x } // should not compile, but does
    }
}
fn main() {}
