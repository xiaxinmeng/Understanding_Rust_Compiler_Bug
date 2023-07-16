
khuey@minbar:~/dev/scratch/rust-scratch$ RUSTFLAGS="-Z drop-tracking" cargo build
   Compiling rust-scratch v0.1.0 (/home/khuey/dev/scratch/rust-scratch)
error: implementation of `std::marker::Send` is not general enough
 --> src/main.rs:7:5
  |
7 | /     send(async {
8 | |         empty().map(ready::<&()>).buffered(0).next().await;
9 | |     });
  | |______^ implementation of `std::marker::Send` is not general enough
  |
  = note: `std::marker::Send` would have to be implemented for the type `&'0 ()`, for any lifetime `'0`...
  = note: ...but `std::marker::Send` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`

error: could not compile `rust-scratch` due to previous error
