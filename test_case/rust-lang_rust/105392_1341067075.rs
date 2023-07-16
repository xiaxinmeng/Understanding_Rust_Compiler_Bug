
error[E0275]: overflow evaluating the requirement `_: std::marker::Sized`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`diesel`)
note: required for `Self` to implement `query_dsl::join_dsl::InternalJoinDsl<_, query_source::joins::Inner, _>`
  --> diesel/src/query_dsl/join_dsl.rs:14:24
   |
14 | impl<T, Rhs, Kind, On> InternalJoinDsl<Rhs, Kind, On> for T
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^
   = note: 127 redundant requirements hidden
   = note: required for `Self` to implement `query_dsl::join_dsl::InternalJoinDsl<_, query_source::joins::Inner, _>`
note: required for `Self` to implement `query_dsl::join_dsl::JoinWithImplicitOnClause<Rhs, query_source::joins::Inner>`
  --> diesel/src/query_dsl/join_dsl.rs:35:22
   |
35 | impl<Lhs, Rhs, Kind> JoinWithImplicitOnClause<Rhs, Kind> for Lhs
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^

For more information about this error, try `rustc --explain E0275`.
error: could not document `diesel`
