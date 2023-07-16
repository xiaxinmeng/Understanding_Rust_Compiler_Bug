
error: lifetime may not live long enough
  --> src/lib.rs:17:12
   |
16 | fn func<'a, 'b, B>(foo: Foo<'a, Bar<'b, B>>) {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
17 |     let _: &Bar<'b, B> = foo.inner();
   |            ^^^^^^^^^^^ type annotation requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`

error[E0597]: `foo` does not live long enough
  --> src/lib.rs:17:26
   |
16 | fn func<'a, 'b, B>(foo: Foo<'a, Bar<'b, B>>) {
   |             -- lifetime `'b` defined here
17 |     let _: &Bar<'b, B> = foo.inner();
   |            -----------   ^^^^^^^^^^^ borrowed value does not live long enough
   |            |
   |            type annotation requires that `foo` is borrowed for `'b`
18 | }
   | - `foo` dropped here while still borrowed
