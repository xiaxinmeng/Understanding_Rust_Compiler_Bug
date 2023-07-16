rust
error[E0185]: method `apply` has a `&self` declaration in the impl, but not in the trait
 --> <anon>:8:5
  |
2 |   fn apply() -> i64;
  |   ------------------ trait declared without `&self`
...
8 |     fn apply(&self) -> i64 { 4 }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `&self` used in impl
