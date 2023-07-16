
error: lifetime may not live long enough
  --> src/lib.rs:12:9
   |
9  | impl<'a> FromStr<'a> for Foo<'a> {
   |      -- lifetime `'a` defined here
10 |     type Err = ();
11 |     fn from_str(s: &str) -> Result<Self, Self::Err> {
   |                    - let's call the lifetime of this reference `'1`
12 |         Ok(Foo(s))
   |         ^^^^^^^^^^ returning this value requires that `'1` must outlive `'a`
