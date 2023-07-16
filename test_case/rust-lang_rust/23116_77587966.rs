 rust
#![allow(dead_code)]
use std::fmt::Debug;
struct S(Box<Debug + 'static>);
impl S {
    fn bar<'a>(&'a mut self)->&'a mut Box<Debug + 'a> {
        let ref mut x = self.0;
        x // should error!
    }
}
fn main() {}
