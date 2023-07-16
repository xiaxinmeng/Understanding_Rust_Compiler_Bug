
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
  --> src/lib.rs:11:12
   |
11 |         Ok(Foo(s))
   |            ^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 10:5...
  --> src/lib.rs:10:5
   |
10 | /     fn from_str(s: &str) -> Result<Self, Self::Err> {
11 | |         Ok(Foo(s))
12 | |     }
   | |_____^
note: ...so that reference does not outlive borrowed content
  --> src/lib.rs:11:16
   |
11 |         Ok(Foo(s))
   |                ^
note: but, the lifetime must be valid for the lifetime `'a` as defined on the impl at 8:6...
  --> src/lib.rs:8:6
   |
8  | impl<'a> FromStr<'a> for Foo<'a> {
   |      ^^
note: ...so that the expression is assignable
  --> src/lib.rs:11:9
   |
11 |         Ok(Foo(s))
   |         ^^^^^^^^^^
   = note: expected  `std::result::Result<Foo<'a>, _>`
              found  `std::result::Result<Foo<'_>, _>`
