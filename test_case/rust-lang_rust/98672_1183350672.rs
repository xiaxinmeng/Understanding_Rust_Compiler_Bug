rust
#![feature(let_else)]

use std::rc::Rc;

pub fn main() {
    {
        // test let-else drops temps after statement
        let rc = Rc::new(0);
        let 0 = *rc.clone() else { unreachable!() };
        Rc::try_unwrap(rc).unwrap();
    }
    {
        // test let-else drops temps before else block
        let rc = Rc::new(0);
        let 1 = *rc.clone() else {
            Rc::try_unwrap(rc).unwrap();
            return;
        };
        unreachable!();
    }
}
