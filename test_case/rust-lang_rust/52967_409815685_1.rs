
warning[E0502]: cannot borrow `stuff.0` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:10
  |
3 |     match stuff {
  |           ----- immutable borrow occurs here
4 |         (ref mut left, _) if *left == "left" => { }
  |          ^^^^^^^^^^^^ mutable borrow occurs here
5 |         _ => {}
  |         - borrow later used here
  |
  = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
          It represents potential unsoundness in your code.
          This warning will become a hard error in the future.
