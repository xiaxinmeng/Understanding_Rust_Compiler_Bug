rust
error: ambiguous glob re-exports
 --> diesel/src/expression/assume_not_null.rs:3:5
  |
3 | use crate::expression::*;
  |     ^^^^^^^^^^^^^^^^^^^^ the name `functions` in the type namespace is first re-exported here
4 | use crate::query_builder::*;
  |     ----------------------- but the name `functions` in the type namespace is also re-exported here
  |
  = note: `#[deny(ambiguous_glob_reexports)]` on by default

error: ambiguous glob re-exports
 --> diesel/src/expression/bound.rs:3:5
  |
3 | use super::*;
  |     ^^^^^^^^ the name `functions` in the type namespace is first re-exported here
4 | use crate::backend::Backend;
5 | use crate::query_builder::*;
  |     ----------------------- but the name `functions` in the type namespace is also re-exported here

error: ambiguous glob re-exports
 --> diesel/src/expression/coerce.rs:4:5
  |
4 | use crate::expression::*;
  |     ^^^^^^^^^^^^^^^^^^^^ the name `functions` in the type namespace is first re-exported here
5 | use crate::query_builder::*;
  |     ----------------------- but the name `functions` in the type namespace is also re-exported here

error: ambiguous glob re-exports
 --> diesel/src/expression/nullable.rs:3:5
  |
3 | use crate::expression::*;
  |     ^^^^^^^^^^^^^^^^^^^^ the name `functions` in the type namespace is first re-exported here
4 | use crate::query_builder::*;
  |     ----------------------- but the name `functions` in the type namespace is also re-exported here

error: ambiguous glob re-exports
 --> diesel/src/expression/sql_literal.rs:4:5
  |
4 | use crate::expression::*;
  |     ^^^^^^^^^^^^^^^^^^^^ the name `functions` in the type namespace is first re-exported here
5 | use crate::query_builder::*;
  |     ----------------------- but the name `functions` in the type namespace is also re-exported here

error: ambiguous glob re-exports
 --> diesel/src/expression/subselect.rs:4:5
  |
4 | use crate::expression::*;
  |     ^^^^^^^^^^^^^^^^^^^^ the name `functions` in the type namespace is first re-exported here
5 | use crate::query_builder::*;
  |     ----------------------- but the name `functions` in the type namespace is also re-exported here

error: ambiguous glob re-exports
  --> diesel/src/query_builder/select_statement/boxed.rs:6:5
   |
6  | use crate::expression::*;
   |     ^^^^^^^^^^^^^^^^^^^^ the name `functions` in the type namespace is first re-exported here
...
19 | use crate::query_builder::*;
   |     ----------------------- but the name `functions` in the type namespace is also re-exported here

error: ambiguous glob re-exports
   --> diesel/src/lib.rs:271:13
    |
271 |     pub use crate::helper_types::*;
    |             ^^^^^^^^^^^^^^^^^^^^^^ the name `max` in the type namespace is first re-exported here
...
274 |     pub use crate::expression::dsl::*;
    |             ------------------------- but the name `max` in the type namespace is also re-exported here

error: ambiguous glob re-exports
   --> diesel/src/lib.rs:271:13
    |
271 |     pub use crate::helper_types::*;
    |             ^^^^^^^^^^^^^^^^^^^^^^ the name `sum` in the type namespace is first re-exported here
...
274 |     pub use crate::expression::dsl::*;
    |             ------------------------- but the name `sum` in the type namespace is also re-exported here

error: ambiguous glob re-exports
   --> diesel/src/lib.rs:271:13
    |
271 |     pub use crate::helper_types::*;
    |             ^^^^^^^^^^^^^^^^^^^^^^ the name `min` in the type namespace is first re-exported here
...
274 |     pub use crate::expression::dsl::*;
    |             ------------------------- but the name `min` in the type namespace is also re-exported here

error: ambiguous glob re-exports
   --> diesel/src/lib.rs:271:13
    |
271 |     pub use crate::helper_types::*;
    |             ^^^^^^^^^^^^^^^^^^^^^^ the name `avg` in the type namespace is first re-exported here
...
274 |     pub use crate::expression::dsl::*;
    |             ------------------------- but the name `avg` in the type namespace is also re-exported here

error: could not compile `diesel` due to 11 previous errors
