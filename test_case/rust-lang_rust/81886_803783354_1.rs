
error: unexpected token: `{
    enum b { }
}`
 --> src/main.rs:2:10
  |
2 | #![doc = {enum b {}}]
  |          ^^^^^^^^^^^

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/hir/map/mod.rs:299:38
