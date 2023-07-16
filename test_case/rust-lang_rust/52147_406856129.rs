text
   Compiling playground v0.0.1 (file:///playground)
error[E0596]: cannot borrow `out` as mutable, as it is a captured variable in a `Fn` closure
  --> src/main.rs:11:13
   |
11 |             out[c.0][c.1] = *v2;
   |             ^^^ cannot borrow as mutable
   |
help: consider changing this to accept closures that implement `FnMut`
  --> src/main.rs:9:44
   |
9  |           v1.par_iter().enumerate().for_each(|(j, v2)| {
   |  ____________________________________________^
10 | |             let c = f(&i, &j);
11 | |             out[c.0][c.1] = *v2;
12 | |         })
   | |_________^

error[E0596]: cannot borrow `out` as mutable, as it is a captured variable in a `Fn` closure
  --> src/main.rs:9:44
   |
9  |         v1.par_iter().enumerate().for_each(|(j, v2)| {
   |                                            ^^^^^^^^^ cannot borrow as mutable
10 |             let c = f(&i, &j);
11 |             out[c.0][c.1] = *v2;
   |             --- mutable borrow occurs due to use of `out` in closure
   |
help: consider changing this to accept closures that implement `FnMut`
  --> src/main.rs:8:39
   |
8  |       m.par_iter().enumerate().for_each(|(i, v1)| {
   |  _______________________________________^
9  | |         v1.par_iter().enumerate().for_each(|(j, v2)| {
10 | |             let c = f(&i, &j);
11 | |             out[c.0][c.1] = *v2;
12 | |         })
13 | |     });
   | |_____^

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.
