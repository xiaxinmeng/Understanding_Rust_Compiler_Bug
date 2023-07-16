
error[E0405]: cannot find trait `Trait` in `A`
 --> src/lib.rs:6:20
  |
6 |     struct A<H: A::Trait>(H);
  |                    ^^^^^ not found in `A`
  |
help: consider importing one of these items
  |
6 |     use crate::A::Trait;
  |

