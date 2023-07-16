plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `borrow_mut` found for struct `std::cell::Ref<'_, BoxedResolver>` in the current scope
    |
    |
320 |     resolver.borrow_mut().access(|resolver| {
    |              ^^^^^^^^^^ method not found in `std::cell::Ref<'_, BoxedResolver>`
   ::: /checkout/library/core/src/borrow.rs:204:8
    |
204 |     fn borrow_mut(&mut self) -> &mut Borrowed;
204 |     fn borrow_mut(&mut self) -> &mut Borrowed;
    |        ---------- the method is available for `std::cell::Ref<'_, BoxedResolver>` here
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use std::borrow::BorrowMut;`

error[E0599]: no method named `clone` found for struct `std::cell::Ref<'_, BoxedResolver>` in the current scope
    |
338 |     resolver.clone()
    |     ---------^^^^^
    |     |        |
    |     |        |
    |     |        this is an associated function, not a method
    |     help: use associated function syntax instead: `std::cell::Ref::<'_, BoxedResolver>::clone`
    |
    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
    = note: the candidate is defined in an impl for the type `std::cell::Ref<'b, T>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc`
