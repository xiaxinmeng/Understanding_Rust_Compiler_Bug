
error[E0275]: overflow evaluating the requirement `T: FromA<U>`
  --> a.rs:41:13
   |
41 |     x.foo(y.to()).to()
   |             ^^
   |
   = note: required because of the requirements on the impl of `FromA<U>` for `T`
   = note: required because of the requirements on the impl of `ToA<T>` for `U`

error: aborting due to previous error
