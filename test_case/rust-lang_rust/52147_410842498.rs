
error[E0596]: cannot borrow `out` as mutable, as it is a captured variable in a `Fn` closure
  --> src/main.rs:11:13
   |
11 |             out[c.0][c.1] = *v2;
   |             ^^^ cannot borrow as mutable
   |
help: the function `for_each` accepts only `Fn` closures, not `FnMut` closures
  --> src/main.rs:9:44
   |
9  |           v1.par_iter().enumerate().for_each(|(j, v2)| {
   |                                     --------
10 | |             let c = f(&i, &j);
11 | |             out[c.0][c.1] = *v2;
12 | |         })
