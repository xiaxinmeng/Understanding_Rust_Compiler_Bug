
error[E0596]: `for_each` does not permit mutating captured variables
  --> src/main.rs:11:13
   |
9  |           v1.par_iter().enumerate().for_each(|(j, v2)| {
   |                                     -------- `for_each` accepts only `Fn` closures
10 |             let c = f(&i, &j);
11 |             out[c.0][c.1] = *v2;
   |             ^^^ cannot borrow as mutable as a `Fn` closure, only `FnMut`
