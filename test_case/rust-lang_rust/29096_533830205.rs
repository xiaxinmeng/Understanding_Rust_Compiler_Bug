
error[E0309]: the parameter type `T` may not live long enough
  --> src/main.rs:33:38
   |
33 | impl<'a, T: Numeric + ToSQL + Clone> WhereType<'a> for T {}
   |          --                          ^^^^^^^^^^^^^
   |          |
   |          help: consider adding an explicit lifetime bound `T: 'a`...
   |
note: ...so that the type `T` will meet its required lifetime bounds
  --> src/main.rs:33:38
   |
33 | impl<'a, T: Numeric + ToSQL + Clone> WhereType<'a> for T {}
   |                                      ^^^^^^^^^^^^^

error[E0277]: the trait bound `&'a Subquery<'a>: Numeric` is not satisfied
  --> src/main.rs:37:10
   |
37 | impl<'a> WhereType<'a> for &'a Subquery<'a> {}
   |          ^^^^^^^^^^^^^ the trait `Numeric` is not implemented for `&'a Subquery<'a>`
   |
   = note: required because of the requirements on the impl of `ToSQL` for `&'a Subquery<'a>`

error[E0277]: `Subquery<'a>` doesn't implement `std::fmt::Display`
  --> src/main.rs:37:10
   |
37 | impl<'a> WhereType<'a> for &'a Subquery<'a> {}
   |          ^^^^^^^^^^^^^ `Subquery<'a>` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Subquery<'a>`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: required because of the requirements on the impl of `std::fmt::Display` for `&'a Subquery<'a>`
   = note: required because of the requirements on the impl of `ToSQL` for `&'a Subquery<'a>`
