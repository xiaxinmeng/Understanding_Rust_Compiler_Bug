
error[E0599]: no method named `try_into` found for struct `S` in the current scope
   --> test.rs:25:7
    |
3   | struct S(i32);
    | -------------- method `try_into` not found for this
...
25  |     a.try_into().unwrap()
    |       ^^^^^^^^ method not found in `S`
    |
   ::: ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:399:8
    |
399 |     fn try_into(self) -> Result<T, Self::Error>;
    |        --------
    |        |
    |        the method is available for `Box<S>` here
    |        the method is available for `Pin<S>` here
    |        the method is available for `Arc<S>` here
    |        the method is available for `Rc<S>` here
    |
    = help: items from traits can only be used if the trait is in scope
help: consider wrapping the receiver expression with the appropriate type
    |
25  |     Box::new(a).try_into().unwrap()
    |     +++++++++ +
help: consider wrapping the receiver expression with the appropriate type
    |
25  |     Pin::new(a).try_into().unwrap()
    |     +++++++++ +
help: consider wrapping the receiver expression with the appropriate type
    |
25  |     Arc::new(a).try_into().unwrap()
    |     +++++++++ +
help: consider wrapping the receiver expression with the appropriate type
    |
25  |     Rc::new(a).try_into().unwrap()
    |     ++++++++ +
help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
    |
3   | use crate::outside::TryInto;
    |
3   | use std::convert::TryInto;
    |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
