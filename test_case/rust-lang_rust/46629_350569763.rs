
error[E0594]: cannot assign to immutable item `x` (Mir)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:25:27
   |
25 |         let _f = to_fn(|| x = 42); //~ ERROR cannot assign
   |                           ^^^^^^ cannot mutate

error[E0596]: cannot borrow immutable item `y` as mutable (Mir)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:28:31
   |
28 |         let _g = to_fn(|| set(&mut y)); //~ ERROR cannot borrow
   |                               ^^^^^^ cannot borrow as mutable

error[E0594]: cannot assign to immutable item `x` (Mir)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:37:32
   |
37 |         let _f = to_fn(move || x = 42); //~ ERROR cannot assign
   |                                ^^^^^^ cannot mutate

error[E0596]: cannot borrow immutable item `y` as mutable (Mir)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:40:36
   |
40 |         let _g = to_fn(move || set(&mut y)); //~ ERROR cannot borrow
   |                                    ^^^^^^ cannot borrow as mutable

error[E0387]: cannot assign to data in a captured outer variable in an `Fn` closure (Ast)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:25:27
   |
25 |         let _f = to_fn(|| x = 42); //~ ERROR cannot assign
   |                           ^^^^^^
   |
help: consider changing this closure to take self by mutable reference
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:25:24
   |
25 |         let _f = to_fn(|| x = 42); //~ ERROR cannot assign
   |                        ^^^^^^^^^

error[E0387]: cannot borrow data mutably in a captured outer variable in an `Fn` closure (Ast)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:28:36
   |
28 |         let _g = to_fn(|| set(&mut y)); //~ ERROR cannot borrow
   |                                    ^
   |
help: consider changing this closure to take self by mutable reference
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:28:24
   |
28 |         let _g = to_fn(|| set(&mut y)); //~ ERROR cannot borrow
   |                        ^^^^^^^^^^^^^^

error[E0387]: cannot assign to data in a captured outer variable in an `Fn` closure (Ast)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:31:55
   |
31 |         let _h = to_fn_mut(|| { set(&mut z); to_fn(|| z = 42); }); //~ ERROR cannot assign
   |                                                       ^^^^^^
   |
help: consider changing this closure to take self by mutable reference
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:31:52
   |
31 |         let _h = to_fn_mut(|| { set(&mut z); to_fn(|| z = 42); }); //~ ERROR cannot assign
   |                                                    ^^^^^^^^^

error[E0594]: cannot assign to captured outer variable in an `Fn` closure (Ast)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:37:32
   |
36 |         let mut x = 0;
   |             ----- help: consider making `mut x` mutable: `mut mut x`
37 |         let _f = to_fn(move || x = 42); //~ ERROR cannot assign
   |                                ^^^^^^
   |
help: consider changing this closure to take self by mutable reference
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:37:24
   |
37 |         let _f = to_fn(move || x = 42); //~ ERROR cannot assign
   |                        ^^^^^^^^^^^^^^

error[E0596]: cannot borrow captured outer variable in an `Fn` closure as mutable (Ast)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:40:41
   |
40 |         let _g = to_fn(move || set(&mut y)); //~ ERROR cannot borrow
   |                                         ^
   |
help: consider changing this closure to take self by mutable reference
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:40:24
   |
40 |         let _g = to_fn(move || set(&mut y)); //~ ERROR cannot borrow
   |                        ^^^^^^^^^^^^^^^^^^^

error[E0594]: cannot assign to captured outer variable in an `Fn` closure (Ast)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:43:65
   |
42 |         let mut z = 0;
   |             ----- help: consider making `mut z` mutable: `mut mut z`
43 |         let _h = to_fn_mut(move || { set(&mut z); to_fn(move || z = 42); }); //~ ERROR cannot assign
   |                                                                 ^^^^^^
   |
help: consider changing this closure to take self by mutable reference
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:43:57
   |
43 |         let _h = to_fn_mut(move || { set(&mut z); to_fn(move || z = 42); }); //~ ERROR cannot assign
   |                                                         ^^^^^^^^^^^^^^

warning: variable does not need to be mutable
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:36:13
   |
36 |         let mut x = 0;
   |             ---^^
   |             |
   |             help: remove this `mut`
   |
   = note: #[warn(unused_mut)] on by default

warning: variable does not need to be mutable
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:39:13
   |
39 |         let mut y = 0;
   |             ---^^
   |             |
   |             help: remove this `mut`

error[E0594]: cannot assign to immutable item `z` (Mir)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:31:55
   |
31 |         let _h = to_fn_mut(|| { set(&mut z); to_fn(|| z = 42); }); //~ ERROR cannot assign
   |                                                       ^^^^^^ cannot mutate

error[E0594]: cannot assign to immutable item `z` (Mir)
  --> /home/ariel/Rust/rust-master/src/test/compile-fail/borrow-immutable-upvar-mutation.rs:43:65
   |
43 |         let _h = to_fn_mut(move || { set(&mut z); to_fn(move || z = 42); }); //~ ERROR cannot assign
   |                                                                 ^^^^^^ cannot mutate

error: aborting due to 12 previous errors
