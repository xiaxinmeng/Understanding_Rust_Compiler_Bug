rust
error[E0408]: variable `a` from pattern #1 is not bound in patterns #2 and #3
  --> file.rs:10:20
   |
10 |         T::T1(a) | T::T2(b) | T::T3(c) | T::T4(a)=> { println!("{:?}", a); }
   |               -    ^^^^^^^^   ^^^^^^^^ pattern doesn't bind `a`
   |               |    |
   |               |    pattern doesn't bind `a`
   |               missing variable

error[E0408]: variable `b` from pattern #2 and variable `c` from pattern #3 are not bound in pattern #1
  --> file.rs:10:26
   |
10 |         T::T1(a) | T::T2(b) | T::T3(c) | T::T4(a)=> { println!("{:?}", a); }
   |         --------         ^          ^ unused variable
   |         |                |
   |         |                unused variable
   |         pattern doesn't bind `b` or `c`
