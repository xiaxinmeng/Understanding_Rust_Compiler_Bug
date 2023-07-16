
error[E0279]: the requirement `for<'a> 'a : 'static` is not satisfied (`expected bound lifetime parameter 'a, found concrete lifetime`)
  --> /home/nmatsakis/tmp/arielb1.rs:21:5
   |
21 |     <u32 as RefFoo>::ref_foo(a)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: required because of the requirements on the impl of `for<'a> Foo` for `&'a u32`
   = note: required because of the requirements on the impl of `RefFoo` for `u32`
note: required by `RefFoo::ref_foo`
  --> /home/nmatsakis/tmp/arielb1.rs:10:5
   |
10 |     fn ref_foo(&self) -> &'static u32;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
