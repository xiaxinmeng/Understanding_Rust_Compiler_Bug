
error[E0617]: can't pass `i8` to variadic function
  --> remainder.rs:10:21
   |
10 |         printf(c_p, value);
   |                     ^^^^^ help: cast the value to `c_int`: `value as c_int`

error: aborting due to previous error
