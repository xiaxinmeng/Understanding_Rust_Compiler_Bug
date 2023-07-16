
warning: the item `panic` is imported redundantly
   --> src\main.rs:187:13
    |
47  | use core::*;
    |     ------- the item `panic` is already imported here
...
187 |         use core::panic;
    |             ^^^^^^^^^^^
    |
    = note: `#[warn(unused_imports)]` on by default

