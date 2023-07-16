
error[E0597]: `i` does not live long enough
 --> src/main.rs:4:12
  |
2 |     let _f = {
  |         -- borrow later stored here
3 |         let i = 1;
4 |         || i
  |         -- ^ borrowed value does not live long enough
  |         |
  |         value captured here
5 |     };
  |     - `i` dropped here while still borrowed
