
error[E0499]: cannot borrow `x` as mutable more than once at a time
  --> src/main.rs:12:13
   |
3  | fn foo<'a>(_: &'a mut S<'a>) {}
   |               ------------- this type requires that the borrow last as long as `S<'a>`, consider using different lifetimes
4  | 
5  | impl<'a> S<'a> {
6  |     fn bar(&mut self) {}
   |            --------- this requires a mutable borrow
...
10 |     let result = foo(&mut x);
   |                      ------ first mutable borrow occurs here
11 |     drop(result);
   |     ------------ `x` is no longer accessible after this call
12 |     let _ = x.bar();
   |             ^ --- this call mutably borrows `x` after it is no longer accessible
   |             |
   |             second mutable borrow occurs here
