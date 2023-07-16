
error[E0275]: overflow evaluating the requirement `(usize, u32): Foo`
  --> <anon>:21:5
   |
21 |     <(usize, u32) as Foo>::whatever();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: required by `Foo::whatever`

error: aborting due to previous error
