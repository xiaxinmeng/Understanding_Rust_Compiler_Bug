
athena. rustc +nightly E0596.rs -Zborrowck=mir
warning: unused variable: `y`
  --> E0596.rs:16:9
   |
16 |     let y = &mut x; //[ast]~ ERROR [E0596]
   |         ^ help: consider using `_y` instead
   |
   = note: #[warn(unused_variables)] on by default

error[E0596]: cannot borrow immutable item `x` as mutable
  --> E0596.rs:16:13
   |
15 |     let x = 1;
   |         - help: consider changing this to be mutable: `mut x`
16 |     let y = &mut x; //[ast]~ ERROR [E0596]
   |             ^^^^^^ cannot borrow as mutable

error: aborting due to previous error
