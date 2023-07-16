
error[E0277]: the trait bound `fn(&()): HR` is not satisfied
  --> src/main.rs:17:8
   |
7  | fn hr<T: HR>(_: T) {}
   |          -- required by this bound in `hr`
...
17 |     hr(not_hr_func);
   |        ^^^^^^^^^^^ the trait `HR` is not implemented for `fn(&())`
   |
   = help: the following implementations were found:
             <for<'r> fn(&'r ()) as HR>

error[E0277]: the trait bound `for<'r> fn(&'r ()): NotHR` is not satisfied
  --> src/main.rs:18:12
   |
11 | fn not_hr<T: NotHR>(_: T) {}
   |              ----- required by this bound in `not_hr`
...
18 |     not_hr(hr_func);
   |            ^^^^^^^ the trait `NotHR` is not implemented for `for<'r> fn(&'r ())`
   |
   = help: the following implementations were found:
             <fn(&'a ()) as NotHR>

error[E0277]: the trait bound `for<'b> fn(&'b ()): NotHR` is not satisfied
  --> src/main.rs:19:12
   |
11 | fn not_hr<T: NotHR>(_: T) {}
   |              ----- required by this bound in `not_hr`
...
19 |     not_hr(hr_func2);
   |            ^^^^^^^^ the trait `NotHR` is not implemented for `for<'b> fn(&'b ())`
   |
   = help: the following implementations were found:
             <fn(&'a ()) as NotHR>
