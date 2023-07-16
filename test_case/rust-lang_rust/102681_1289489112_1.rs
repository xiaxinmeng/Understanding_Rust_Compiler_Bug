
error[[E0760]](https://doc.rust-lang.org/nightly/error-index.html#E0760): `async fn` return type cannot contain a projection or `Self` that references lifetimes from a parent scope
  --> src/lib.rs:12:23
   |
12 |     async fn foo() -> <Self as Trait<'a>>::Assoc {}
   |                       ^----^^^^^^^^^^^^^^^^^^^^^
   |                        |
   |                        help: consider spelling out the type instead: `Foo<'a>`

error: lifetime may not live long enough
  --> src/lib.rs:12:50
   |
11 | impl<'a> Foo<'a> {
   |      -- lifetime `'a` defined here
12 |     async fn foo() -> <Self as Trait<'a>>::Assoc {}
   |                                                  ^^ returning this value requires that `'a` must outlive `'static`

For more information about this error, try `rustc --explain E0760`.
