
error: lifetime may not live long enough
 --> foo.rs:8:5
  |
7 | fn bar(foo: &mut Foo<'_>) {
  |             -        -- let's call this `'2`
  |             |
  |             let's call the lifetime of this reference `'1`
8 |     foo.modify();
  |     ^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`

error: aborting due to previous error
