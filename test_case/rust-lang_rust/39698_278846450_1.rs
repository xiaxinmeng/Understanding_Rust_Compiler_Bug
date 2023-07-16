
error[E0408]: variable `b` from pattern #2 is not bound in pattern #1
  --> file.rs:10:26
   |
10 |         T::T1(a) | T::T2(b) | T::T3(c) | T::T4(a)=> { println!("{:?}", a); }
   |                          ^ pattern doesn't bind `b`
