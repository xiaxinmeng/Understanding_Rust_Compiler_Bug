rust
error[E0408]: variable `a` from pattern #1 is not bound in pattern #2
  --> file.rs:10:20
   |
10 |         T::T1(a) | T::T2(b) | T::T3(c) | T::T4(a)=> { println!("{:?}", a); }
   |                    ^^^^^^^^ pattern doesn't bind `a`

error[E0408]: variable `b` from pattern #2 is not bound in pattern #1
  --> file.rs:10:26
   |
10 |         T::T1(a) | T::T2(b) | T::T3(c) | T::T4(a)=> { println!("{:?}", a); }
   |                          ^ pattern doesn't bind `b`

error[E0408]: variable `a` from pattern #1 is not bound in pattern #3
  --> file.rs:10:31
   |
10 |         T::T1(a) | T::T2(b) | T::T3(c) | T::T4(a)=> { println!("{:?}", a); }
   |                               ^^^^^^^^ pattern doesn't bind `a`

error[E0408]: variable `c` from pattern #3 is not bound in pattern #1
  --> file.rs:10:37
   |
10 |         T::T1(a) | T::T2(b) | T::T3(c) | T::T4(a)=> { println!("{:?}", a); }
   |                                     ^ pattern doesn't bind `c`
