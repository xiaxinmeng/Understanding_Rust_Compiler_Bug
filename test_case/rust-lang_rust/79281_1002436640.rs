
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
 --> src/lib.rs:8:49
  |
8 |     fn use_pattern(&self, pattern: &Pattern) -> impl Iterator<Item = Foo<'_>> {
  |                                    --------     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |                                    |
  |                                    hidden type `Map<Filter<std::slice::Iter<'_, String>, [closure@src/lib.rs:11:21: 11:59]>, [closure@src/lib.rs:12:18: 12:45]>` captures the anonymous lifetime defined here
  |
help: to declare that the `impl Trait` captures `'_`, you can add an explicit `'_` lifetime bound
  |
8 |     fn use_pattern(&self, pattern: &Pattern) -> impl Iterator<Item = Foo<'_>> + '_ {
  |                                                                               ++++
