rust
error[E0408]: variable `a` from pattern #1 is not bound in pattern #2
 --> pat.rs:8:20
  |
8 |         T::T1(a) | T::T2() => { println!("{:?}", a); }
  |                    ^^^^^^^ pattern doesn't bind `a`
