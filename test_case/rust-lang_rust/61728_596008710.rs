
error: lifetime may not live long enough
 --> src/lib.rs:4:14
  |
3 | fn mut_ref<'a, 'b>(val: &'a mut &'b mut Foo)  {
  |            --  -- lifetime `'b` defined here
  |            |
  |            lifetime `'a` defined here
4 |     let tmp: &'b mut Foo = *val;
  |              ^^^^^^^^^^^ type annotation requires that `'a` must outlive `'b`
  |
  = help: consider adding the following bound: `'a: 'b`
