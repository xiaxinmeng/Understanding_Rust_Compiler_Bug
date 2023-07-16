text
error[E0596]: cannot borrow `out` as mutable, as it is a captured variable in a `Fn` closure
  --> src/main.rs:10:13
   |
10 |             out[c.0][c.1] = *v2;
   |             ^^^ cannot borrow as mutable
   |
help: consider changing this to accept closures that implement `FnMut`
  --> src/main.rs:8:44
   |
8  |           v1.par_iter().enumerate().for_each(|(j, v2)| {
   |  ____________________________________________^
9  | |             let c = f(&i, &j);
10 | |             out[c.0][c.1] = *v2;
11 | |         })
   | |_________^

error[E0596]: cannot borrow `out` as mutable, as it is a captured variable in a `Fn` closure
  --> src/main.rs:8:44
   |
8  |         v1.par_iter().enumerate().for_each(|(j, v2)| {
   |                                            ^^^^^^^^^ cannot borrow as mutable
9  |             let c = f(&i, &j);
10 |             out[c.0][c.1] = *v2;
   |             --- mutable borrow occurs due to use of `out` in closure
   |
help: consider changing this to accept closures that implement `FnMut`
  --> src/main.rs:7:39
   |
7  |       m.par_iter().enumerate().for_each(|(i, v1)| {
   |  _______________________________________^
8  | |         v1.par_iter().enumerate().for_each(|(j, v2)| {
9  | |             let c = f(&i, &j);
10 | |             out[c.0][c.1] = *v2;
11 | |         })
12 | |     });
   | |_____^

error: aborting due to 2 previous errors

