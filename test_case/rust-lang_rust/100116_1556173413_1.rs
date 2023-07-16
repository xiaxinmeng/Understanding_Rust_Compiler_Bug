
error[[E0277]](https://doc.rust-lang.org/stable/error_codes/E0277.html): the trait bound `S: From<fn() {f1}>` is not satisfied
  --> src/main.rs:15:22
   |
15 |     let s2 = S::from(f1);
   |              ------- ^^ the trait `From<fn() {f1}>` is not implemented for `S`
   |              |
   |              required by a bound introduced by this call
   |
   = help: the trait `From<fn()>` is implemented for `S`
