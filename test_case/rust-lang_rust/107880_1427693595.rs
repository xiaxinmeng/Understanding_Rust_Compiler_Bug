rust
error: ambiguous glob re-exports
   --> diesel/src/lib.rs:271:13
    |
271 |     pub use crate::helper_types::*;
    |             ^^^^^^^^^^^^^^^^^^^^^^ the name `max` in the type namespace is first re-exported here
...
274 |     pub use crate::expression::dsl::*;
    |             ------------------------- but the name `max` in the type namespace is also re-exported here
    |
    = note: `#[deny(ambiguous_glob_reexports)]` on by default

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
    |             ^^^^^^^^^^^^^^^^^^^^^^ the name `sum` in the type namespace is first re-exported here
...
274 |     pub use crate::expression::dsl::*;
    |             ------------------------- but the name `sum` in the type namespace is also re-exported here

error: ambiguous glob re-exports
   --> diesel/src/lib.rs:271:13
    |
271 |     pub use crate::helper_types::*;
    |             ^^^^^^^^^^^^^^^^^^^^^^ the name `avg` in the type namespace is first re-exported here
...
274 |     pub use crate::expression::dsl::*;
    |             ------------------------- but the name `avg` in the type namespace is also re-exported here

error: could not compile `diesel` due to 4 previous errors
