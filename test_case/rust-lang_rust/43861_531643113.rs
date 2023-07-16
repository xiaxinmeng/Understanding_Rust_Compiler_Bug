
error[E0308]: mismatched types
 --> file2.rs:7:27
  |
7 |   opt.map(|arg| takes_ref(arg));
  |       ---                 ^^^ expected &Foo, found struct `Foo`
  |       |
  |       help: consider using `as_ref` instead: `as_ref().map`
  |
  = note: expected type `&Foo`
             found type `Foo`
