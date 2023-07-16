
warning: outlives requirements can be inferred
 --> <source>:4:25
  |
4 |         struct Foo<'a, T: 'a>(&'a T);
  |                         ^ help: remove this bound
