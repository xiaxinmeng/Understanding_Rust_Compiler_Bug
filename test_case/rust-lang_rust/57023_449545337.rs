
error[E0432]: unresolved import `crate::NON_EXISTENT`
 --> src/lib.rs:7:12
  |
7 | use crate::NON_EXISTENT::ANY_GIBERISH_HERE_CAUSES_PANIC;
  |            ^^^^^^^^^^^^ maybe a missing `extern crate NON_EXISTENT;`?

warning: unused import: `crate::NON_EXISTENT::ANY_GIBERISH_HERE_CAUSES_PANIC`
 --> src/lib.rs:7:5
  |
7 | use crate::NON_EXISTENT::ANY_GIBERISH_HERE_CAUSES_PANIC;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

error: aborting due to previous error
