 rust
test.rs:11:18: 11:24 error: the trait `core::marker::Sync` is not implemented for the type `alloc::rc::Rc<u8>` [E0277]
test.rs:11      let guard = scoped(|| {
                            ^~~~~~
note: in expansion of closure expansion
test.rs:9:22: 32:3 note: expansion site
test.rs:11:18: 11:24 note: `alloc::rc::Rc<u8>` cannot be shared between threads safely
test.rs:11      let guard = scoped(|| {
                            ^~~~~~
note: in expansion of closure expansion
test.rs:9:22: 32:3 note: expansion site
error: aborting due to previous error
