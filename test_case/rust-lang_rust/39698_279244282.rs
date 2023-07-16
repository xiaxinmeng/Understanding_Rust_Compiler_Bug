
error[E0408]: variable `a` from pattern #1 is not bound in patterns #2 and #3
  --> file.rs:10:20
   |
10 |         T::T1(a) | T::T2(b) | T::T3(c) | T::T4(a)=> { println!("{:?}", a); }
   |               ^    --------   -------- pattern doesn't bind `a`
   |               |    |
   |               |    pattern doesn't bind `a`
   |               variable not in all patterns
