text
error[E0382]: use of moved value: `self`
 --> src/main.rs:9:20
  |
8 |         Self::partial(self.1);
  |                       ------ value moved here
9 |         Self::full(self);
  |                    ^^^^ value used here after partial move
  |
  = note: move occurs because `self.1` has type `S`, which does not implement the `Copy` trait
