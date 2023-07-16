rust
#![feature(existential_type, untagged_unions)]

#[macro_use]
extern crate lazy_static;

use std::ops::Deref;

#[derive(Debug)]
struct ExpensiveResult;

// TODO: move this inside of `lazy_local!`.
existential type Init: FnOnce() -> ExpensiveResult;

macro_rules! lazy_local {
    (ref $name:ident : $ty:ty = $init:expr;) => {
        #[allow(unions_with_drop_fields)]
        union BrieflyUninit {
            uninit: (),
            // If $name is never deref'd, this initializer is never dropped.
            value: Init,
        }

        static mut INIT: BrieflyUninit = BrieflyUninit { uninit: () };
        let init = move || -> Init { move || $init };
        unsafe {
            std::ptr::write(&mut INIT.value, init());
        }

        lazy_static! {
            static ref $name: $ty = unsafe { std::ptr::read(&INIT.value)() };
        }
    };
}

fn lazy_deref_with_args(arg: i32) -> &'static impl Deref<Target = ExpensiveResult> {
    lazy_local! {
        ref STATE: ExpensiveResult = { println!("arg={}", arg); ExpensiveResult };
    }

    &STATE
}

fn main() {
    let state = lazy_deref_with_args(1);
    println!("{:?}", **state);
}
