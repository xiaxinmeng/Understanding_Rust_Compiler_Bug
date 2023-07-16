
error[E0107]: missing generics for enum `Option`
   --> <source>:1:14
    |
1   | fn func2(_x: Option<Target = Option>) {}
    |              ^^^^^^ expected 1 type argument
    |
note: enum defined here, with 1 type parameter: `T`
help: use angle brackets to add missing type argument
    |
1   | fn func2(_x: Option<T><Target = Option>) {}
    |                    ^^^

error[E0107]: missing generics for enum `Option`
   --> <source>:1:30
    |
1   | fn func2(_x: Option<Target = Option>) {}
    |                              ^^^^^^ expected 1 type argument
    |
note: enum defined here, with 1 type parameter: `T`
help: use angle brackets to add missing type argument
    |
1   | fn func2(_x: Option<Target = Option<T>>) {}
    |                                    ^^^

error[E0229]: associated type bindings are not allowed here
 --> <source>:1:21
  |
1 | fn func2(_x: Option<Target = Option>) {}
  |                     ^^^^^^^^^^^^^^^ associated type not allowed here

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0107, E0229.
For more information about an error, try `rustc --explain E0107`.
