
error[E0596]: cannot borrow `y` as mutable, as it is a captured variable in a `Fn` closure
 --> src/main.rs:5:62
  |
5 |     (0..10).into_par_iter().for_each(|x| println!("{}{}", x, &mut y));
  |                                                              ^^^^^^ cannot borrow as mutable
