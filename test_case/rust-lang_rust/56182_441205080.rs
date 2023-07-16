
error: `actix` import is ambiguous
 --> src\db.rs:2:5
  |
2 | use actix::prelude::*;
  |     ^^^^^------------
  |     |
  |     can refer to external crate `::actix`
  |     may refer to `self::actix` in the future
  |
  = help: write `::actix` or `self::actix` explicitly instead
  = note: in the future, `#![feature(uniform_paths)]` may become the default
