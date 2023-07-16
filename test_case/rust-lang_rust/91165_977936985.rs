
error[E0599]: no variant or associated item named `BadVariant` found for enum `Foo` in the current scope
  --> src/main.rs:10:14
   |
1  | enum Foo {
   | -------- variant or associated item `BadVariant` not found here
...
10 |         Foo::BadVariant => { /* Big Yikes */ },
   |              ^^^^^^^^^^ variant or associated item not found in `Foo`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hang` due to previous error
