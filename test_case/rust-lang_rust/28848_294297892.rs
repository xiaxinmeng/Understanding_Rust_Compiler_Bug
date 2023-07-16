
error[E0478]: lifetime bound not satisfied
  --> /tmp/t:10:5
   |
10 |     Foo::<'a, 'b>::xmute(u)
   |     ^^^^^^^^^^^^^^^^^^^^
   |
note: lifetime parameter instantiated with the lifetime 'b as defined on the body at 9:40
  --> /tmp/t:9:41
   |
9  |   pub fn foo<'a, 'b>(u: &'b ()) -> &'a () {
   |  _________________________________________^ starting here...
10 | |     Foo::<'a, 'b>::xmute(u)
11 | | }
   | |_^ ...ending here
note: but lifetime parameter must outlive the lifetime 'a as defined on the body at 9:40
  --> /tmp/t:9:41
   |
9  |   pub fn foo<'a, 'b>(u: &'b ()) -> &'a () {
   |  _________________________________________^ starting here...
10 | |     Foo::<'a, 'b>::xmute(u)
11 | | }
   | |_^ ...ending here

error: aborting due to previous error
