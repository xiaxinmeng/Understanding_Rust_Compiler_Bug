plain
   Compiling toml v0.5.7
error[E0308]: mismatched types
   --> src/bootstrap/dist.rs:826:18
    |
826 |         run.path(&["src", "rust-src"])
    |                  ^^^^^^^^^^^^^^^^^^^^ expected `str`, found array `[&str; 2]`
    = note: expected reference `&str`
    = note: expected reference `&str`
               found reference `&[&str; 2]`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:00:53
