
error[E0277]: the trait bound `*mut u8: std::marker::Sync` is not satisfied in `Inner`
  --> test.rs:15:5
   |
15 |     is_send(arc);
   |     ^^^^^^^ `*mut u8` cannot be shared between threads safely
   |
   = help: within `Inner`, the trait `std::marker::Sync` is not implemented for `*mut u8`
   = note: required because it appears within the type `Inner`
   = note: required because of the requirements on the impl of `std::marker::Send` for `alloc::arc::Arc<Inner>`
   = note: required by `is_send`

error: aborting due to previous error(s)
