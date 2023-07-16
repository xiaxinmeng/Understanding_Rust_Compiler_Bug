
> cargo +nightly build --target=i686-pc-windows-msvc
   Compiling dependency v0.1.0 (C:\Users\ryanl\Code\issue-repros\rustc-82991\dependency)
   Compiling main_crate v0.1.0 (C:\Users\ryanl\Code\issue-repros\rustc-82991\main_crate)
error: found crates (`dependency` and `dependency`) with colliding StableCrateId values.
 --> src\main.rs:2:5
  |
2 |     dep::foo!();
  |     ^^^

error: aborting due to previous error

error: could not compile `main_crate

> cargo +nightly build 
   Compiling main_crate v0.1.0 (C:\Users\ryanl\Code\issue-repros\rustc-82991\main_crate)
    Finished dev [unoptimized + debuginfo] target(s) in 0.51s
