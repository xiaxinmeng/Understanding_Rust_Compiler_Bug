 rust
use std::cell::RefCell;

fn main() {
    let c = RefCell::new(vec![]);
    let mut y = 1u;
    c.push(|| y = 0);
    c.push(|| y = 0);
    // confliciting borrow detected when not going through trait
    // c.borrow_mut().push(|| y = 0);
    // c.borrow_mut().push(|| y = 0);
}

trait Push<'c> {
    fn push<'f: 'c>(&self, push: ||:'f -> ());
}

impl<'c> Push<'c> for RefCell<Vec<||:'c>> {
    fn push<'f: 'c>(&self, fun: ||:'f -> ()) {
        self.borrow_mut().push(fun)
    }
}
