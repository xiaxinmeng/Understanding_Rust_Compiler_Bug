plain
   Compiling heck v0.3.1
   Compiling structopt-derive v0.4.18
    Checking thiserror v1.0.30
    Checking structopt v0.3.25
error[E0277]: a value of type `TokenStream` cannot be built from an iterator over elements of type `&TokenTree`
     |
     |
229  |         if let success @ Some(..) = format_lazy_static(context, shape, ts.trees().collect()) {
     |                                     ------------------                 ^^^^^^^^^^^^^^^^^^^^ value of type `TokenStream` cannot be built from `std::iter::Iterator<Item=&TokenTree>`
     |                                     required by a bound introduced by this call
     |
     |
     = help: the trait `FromIterator<&TokenTree>` is not implemented for `TokenStream`
note: required by a bound in `std::iter::Iterator::collect`
     |
     |
1784 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `std::iter::Iterator::collect`
error[E0308]: mismatched types
   --> src/tools/rustfmt/src/macros.rs:875:25
    |
875 |                         span,
875 |                         span,
    |                         ^^^^ expected struct `Span`, found `&Span`
help: consider dereferencing the borrow
    |
    |
875 |                         span: *span,

error[E0308]: mismatched types
   --> src/tools/rustfmt/src/macros.rs:882:44
    |
    |
882 |                     self.add_meta_variable(&mut iter)?;
    |                                            ^^^^^^^^^ expected struct `rustc_ast::tokenstream::Cursor`, found struct `CursorRef`
    = note: expected mutable reference `&mut rustc_ast::tokenstream::Cursor`
    = note: expected mutable reference `&mut rustc_ast::tokenstream::Cursor`
               found mutable reference `&mut CursorRef<'_>`
error[E0308]: mismatched types
   --> src/tools/rustfmt/src/macros.rs:899:56
    |
    |
899 |                         self.add_repeat(delimited_arg, delimited, &mut iter)?;
    |                                                        ^^^^^^^^^ expected enum `Delimiter`, found `&Delimiter`
help: consider dereferencing the borrow
    |
    |
899 |                         self.add_repeat(delimited_arg, *delimited, &mut iter)?;

error[E0308]: mismatched types
   --> src/tools/rustfmt/src/macros.rs:899:67
    |
    |
899 |                         self.add_repeat(delimited_arg, delimited, &mut iter)?;
    |                                                                   ^^^^^^^^^ expected struct `rustc_ast::tokenstream::Cursor`, found struct `CursorRef`
    = note: expected mutable reference `&mut rustc_ast::tokenstream::Cursor`
    = note: expected mutable reference `&mut rustc_ast::tokenstream::Cursor`
               found mutable reference `&mut CursorRef<'_>`
error[E0308]: mismatched types
   --> src/tools/rustfmt/src/macros.rs:902:59
    |
    |
902 |                         self.add_delimited(delimited_arg, delimited);
    |                                                           ^^^^^^^^^ expected enum `Delimiter`, found `&Delimiter`
help: consider dereferencing the borrow
    |
    |
902 |                         self.add_delimited(delimited_arg, *delimited);

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustfmt-nightly` due to 6 previous errors
