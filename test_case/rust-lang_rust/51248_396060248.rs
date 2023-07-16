
[01:20:32] error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
[01:20:32]    --> librustc_driver/test.rs:187:31
[01:20:32]     |
[01:20:32] 187 | const D2: ty::DebruijnIndex = D1.shifted_in(1);
[01:20:32]     |                               ^^^^^^^^^^^^^^^^
