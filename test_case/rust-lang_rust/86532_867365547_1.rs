
help:  try removing the generic parameter and using `impl Trait` instead
  |
8 -         fn foo<U: Debug>(&self, _: &U) { }
8 +         fn foo(&self, _: &impl Debug) { }
  |              ><           ^^^^^^^^^^
