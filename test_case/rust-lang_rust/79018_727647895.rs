rust
#![feature(const_evaluatable_checked)]

use what::{Const, Trait};

fn main() {
    let _ = Const::<1>::assoc_fn();
}
