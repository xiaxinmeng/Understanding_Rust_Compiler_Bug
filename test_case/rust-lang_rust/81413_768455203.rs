
error[E0432]: unresolved import `doesnt_exist`
 --> <source>:8:9
  |
8 | pub use doesnt_exist::*;
  |         ^^^^^^^^^^^^ maybe a missing crate `doesnt_exist`?

warning: unused imports: `ITEM`, `Item`, `item`
  --> <source>:11:17
   |
11 |     use crate::{item, Item, ITEM};
   |                 ^^^^  ^^^^  ^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `crate::item`
  --> <source>:15:9
   |
15 |     use crate::item;
   |         ^^^^^^^^^^^

warning: unused import: `crate::Item`
  --> <source>:16:9
   |
16 |     use crate::Item;
   |         ^^^^^^^^^^^

warning: unused import: `crate::ITEM`
  --> <source>:17:9
   |
17 |     use crate::ITEM;
   |         ^^^^^^^^^^^

warning: unused import: `crate::item`
  --> <source>:21:9
   |
21 |     use crate::item;
   |         ^^^^^^^^^^^

error: aborting due to previous error
