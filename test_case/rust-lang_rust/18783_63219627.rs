 rust
// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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

fn ufcs() {
    let c = RefCell::new(vec![]);
    let mut y = 1u;

    Push::push(&c, || y = 0);
    Push::push(&c, || y = 0);
}

trait Push<'c> {
    fn push<'f: 'c>(&self, push: ||:'f -> ());
}

impl<'c> Push<'c> for RefCell<Vec<||:'c>> {
    fn push<'f: 'c>(&self, fun: ||:'f -> ()) {
        self.borrow_mut().push(fun)
    }
}
