
test.rs:6:25: 6:34 error: mismatched types:
 expected `core::option::Option<_>`,
    found `alloc::rc::Rc<(T, core::cell::RefCell<core::option::Option<_>>)>`
(expected enum `core::option::Option`,
    found struct `alloc::rc::Rc`) [E0308]
test.rs:6     *x.1.borrow_mut() = x.clone();
                                  ^~~~~~~~~
