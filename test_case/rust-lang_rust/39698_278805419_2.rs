rust
error[E0408]: variable `a` from pattern #1 is not bound in pattern #2
 --> file.rs:8:20
  |
8 |         T::T1(a) | T::T2(b) => { println!("{:?}", a); }
  |               -    ^^^^^^^^ pattern doesn't bind `a`
  |               |
  |               unbound variable

error[E0408]: variable `b` from pattern #2 is not bound in pattern #1
 --> file.rs:8:26
  |
8 |         T::T1(a) | T::T2(b) => { println!("{:?}", a); }
  |         --------         ^ unbound variable
  |         |
  |         pattern doesn't bind `b`
