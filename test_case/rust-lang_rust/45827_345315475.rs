rust
// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//revisions: ast mir
//[mir] compile-flags: -Z emit-end-regions -Z borrowck-mir -Z nll

use std::cell::Cell;

fn assert_sync<T>(x: T) -> T where T: Sync { x }
struct AssertSync<T>(T) where T: Sync;

fn foo() {
    let _x = assert_sync(Cell::new(()));
    //[ast]~^ ERROR the trait bound `std::cell::Cell<()>: std::marker::Sync` is not satisfied [E0277]
    //[mir]~^^ ERROR the trait bound `std::cell::Cell<()>: std::marker::Sync` is not satisfied [E0277]
    let _x = AssertSync(Cell::new(()));
    //[ast]~^ ERROR the trait bound `std::cell::Cell<()>: std::marker::Sync` is not satisfied [E0277]
    //[ast]~^^ ERROR the trait bound `std::cell::Cell<()>: std::marker::Sync` is not satisfied [E0277]
    //[mir]~^^^ ERROR the trait bound `std::cell::Cell<()>: std::marker::Sync` is not satisfied [E0277]
    //[mir]~^^^^ ERROR the trait bound `std::cell::Cell<()>: std::marker::Sync` is not satisfied [E0277]
}

fn main() {}
