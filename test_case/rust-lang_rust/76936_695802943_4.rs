text
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
  --> src/main.rs:9:14
   |
7  |     let p1 = &x;
   |              -- immutable borrow occurs here
8  |     let at_ft_mut = unsafe { p1.assume_unique() };
9  |     let p2 = &mut x;
   |              ^^^^^^ mutable borrow occurs here
10 |     let at_ft_mut_2 = p2.get_mut();
11 |     mem::swap(at_ft_mut, at_ft_mut_2);
   |               --------- immutable borrow later used here
