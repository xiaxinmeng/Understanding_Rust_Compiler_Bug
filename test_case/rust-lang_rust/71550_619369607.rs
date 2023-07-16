
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter 'a in function call due to conflicting requirements
  --> <source>:25:5
   |
25 |     bad(&Bar(PhantomData), x)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the body at 24:45...
  --> <source>:24:46
   |
24 |   pub fn extend<'a, T>(x: &'a T) -> &'static T {
   |  ______________________________________________^ starting here...
25 | |     bad(&Bar(PhantomData), x)
26 | | }
   | |_^ ...ending here
note: ...so that reference does not outlive borrowed content
  --> <source>:25:28
   |
25 |     bad(&Bar(PhantomData), x)
   |                            ^
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that types are compatible (expected &'static T, found &T)
  --> <source>:25:9
   |
25 |     bad(&Bar(PhantomData), x)
   |         ^^^^^^^^^^^^^^^^^

error: aborting due to previous error

Compiler returned: 101
