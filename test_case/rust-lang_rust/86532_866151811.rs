
help: try removing the generic parameter and using `impl Trait` instead
  |
8 |         fn foo<U: Debug>(&self, _: &impl Debug) { }
  |               xxxxxxxxxx            ++++++++++
