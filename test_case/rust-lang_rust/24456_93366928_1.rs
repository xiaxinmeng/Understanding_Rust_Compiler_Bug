
test.rs:6:25: 6:40 error: mismatched types:
 expected `core::option::Option<_>`,
    found `core::option::Option<alloc::rc::Rc<(T, core::cell::RefCell<core::option::Option<_>>)>>`
(cyclic type of infinite size) [E0308]
test.rs:6     *x.1.borrow_mut() = Some(x.clone());
                                  ^~~~~~~~~~~~~~~
