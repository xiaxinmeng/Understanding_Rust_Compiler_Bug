
warning[E0170]: pattern binding `A` is named the same as one of the variants of the type `Foo`
  --> <source>:14:9
   |
14 |         A => unsafe {
   |         ^ help: to match on the variant, qualify the path: `Foo::A`
   |
   = note: `#[warn(bindings_with_variant_name)]` on by default

warning: unreachable pattern
  --> <source>:21:9
   |
14 |         A => unsafe {
   |         - matches any value
...
21 |         Foo::B(x) => unsafe {
   |         ^^^^^^^^^ unreachable pattern
   |
   = note: `#[warn(unreachable_patterns)]` on by default

warning: unused variable: `A`
  --> <source>:14:9
   |
14 |         A => unsafe {
   |         ^ help: if this is intentional, prefix it with an underscore: `_A`
   |
   = note: `#[warn(unused_variables)]` on by default
