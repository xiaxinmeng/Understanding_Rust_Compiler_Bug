
error[E0275]: overflow evaluating the requirement `(): Deserialize<__D>`
  --> <source>:13:11
   |
13 | impl<__D> Deserialize<__D> for ()
   |           ^^^^^^^^^^^^^^^^
   |
note: required by a bound in `Deserialize`
  --> <source>:10:1
   |
10 | pub trait Deserialize<D> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Deserialize`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
