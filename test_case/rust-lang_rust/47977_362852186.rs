
RUSTFLAGS=-Znll cargo +nightly test
   Compiling bitflags v1.0.1 (file:///home/bob_twinkles/Code/rust/bitflags)
error[E0503]: cannot use `m1` because it was mutably borrowed
   --> src/lib.rs:943:15
    |
943 |         m1 -= m1;
    |         --    ^^ use of borrowed `m1`
    |         |
    |         borrow of `m1` occurs here

error: aborting due to previous error

error: Could not compile `bitflags`.
