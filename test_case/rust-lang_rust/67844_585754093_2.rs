
error: cannot infer an appropriate lifetime
  --> src/main.rs:11:5
   |
6  |  = impl Fn<(A,), Output = impl FnOnce(B) -> C + 'a> + 'a;
   |                           ------------------------ this return type evaluates to the `'static` lifetime...
...
11 |     move |a| move |b| f(a,b)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ ...but this borrow...
   |
note: ...can't outlive the lifetime `'a` as defined on the function body at 8:10
  --> src/main.rs:8:10
   |
8  | fn curry<'a, A: 'a, B, C, F: Fn(A, B) -> C> (f: &'a F)
   |          ^^
help: you can add a bound to the return type to make it last less than `'static` and match the lifetime `'a` as defined on the function body at 8:10
   |
9  |     -> Curried<'a, A, B, C, F> + '_
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: cannot infer an appropriate lifetime
  --> src/main.rs:11:14
   |
6  |  = impl Fn<(A,), Output = impl FnOnce(B) -> C + 'a> + 'a;
   |                           ------------------------ this return type evaluates to the `'static` lifetime...
...
11 |     move |a| move |b| f(a,b)
   |              ^^^^^^^^^^^^^^^ ...but this borrow...
   |
note: ...can't outlive the lifetime `'a` as defined on the function body at 8:10
  --> src/main.rs:8:10
   |
8  | fn curry<'a, A: 'a, B, C, F: Fn(A, B) -> C> (f: &'a F)
   |          ^^
help: you can add a bound to the return type to make it last less than `'static` and match the lifetime `'a` as defined on the function body at 8:10
   |
9  |     -> Curried<'a, A, B, C, F> + '_
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors
