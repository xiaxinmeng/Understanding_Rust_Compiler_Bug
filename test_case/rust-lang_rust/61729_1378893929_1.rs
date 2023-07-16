
error[E0576]: cannot find method or associated constant `bar` in `Foo`
  --> example.rs:11:19
   |
11 |     <Foo as Foo>::bar()
   |                   ^^^ not found in `Foo`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0576`.
