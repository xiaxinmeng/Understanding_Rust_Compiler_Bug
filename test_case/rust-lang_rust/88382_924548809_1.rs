
error[E0631]: type mismatch in function arguments
  --> src/main.rs:38:47
   |
28 | fn add(x: &i32, y: &i32) -> i32 {
   | ------------------------------- found signature of `for<'r, 's> fn(&'r i32, &'s i32) -> _`
...
38 |     let expr2 = Func::<i32, i32, i32, _>::new(add);
   |                 ----------------------------- ^^^ expected signature of `for<'r, 's> fn(<i32 as Scalar>::Ref<'r>, <i32 as Scalar>::Ref<'s>) -> _`
   |                 |
   |                 required by a bound introduced by this call
   |
