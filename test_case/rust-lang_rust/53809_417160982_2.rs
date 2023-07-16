
error[E0597]: `x` does not live long enough
 --> src/main.rs:8:21
  |
8 |     c = (*b).map(|x|x.as_slice());
  |                     ^          - `x` dropped here while still borrowed
  |                     |
  |                     borrowed value does not live long enough

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:8:9
  |
8 |     c = (*b).map(|x|x.as_slice());
  |         ^^^^ cannot move out of borrowed content
