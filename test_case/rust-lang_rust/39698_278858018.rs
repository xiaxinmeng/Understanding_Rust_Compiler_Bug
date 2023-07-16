rust
error[E0408]: multiple variables not present in all patterns
  --> $DIR/issue-39698.rs:20:37
   |
20 |         T::T1(a, d) | T::T2(d, b) | T::T3(c) | T::T4(a) => { println!("{:?}", a); }
   |         ------^--^-   ---------^-   ------^-
   |               |  |             |          |
   |               |  |             |          variable not in all patterns
   |               |  |             variable not in all patterns
   |               |  variable not in all patterns
   |               variable not in all patterns

error: aborting due to 1 previous errors
