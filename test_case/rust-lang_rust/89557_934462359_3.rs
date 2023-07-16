
error[E0499]: cannot borrow `self.0` as mutable more than once at a time
  --> src/lib.rs:14:18
9  |             fill(&mut self.0);
   |                  ^^^^^^^^^^^ first mutable borrow occurs here
10 |             A::arbitrary(&mut self.0);
   |                               ^^^^^^ second mutable borrow occurs here
   |
help: consider making `Arbitrary`'s lifetimes shorter-lived
   |
6  | pub trait Arbitrary<'a> {
   |                    ~~~~ remove this
7  |    fn arbitrary(u: &'a mut String);
   |                     ~~~ remove this
