rust
  Compiling rustc v0.0.0 (file:///C:/projects/rust/src/librustc)
error: unused macro definition
   --> src\librustc\util\common.rs:119:1
    |
119 | / macro_rules! option_try(
120 | |     ($e:expr) => (match $e { Some(e) => e, None => return None })
121 | | );
    | |__^
    |
    = note: #[deny(unused_macros)] implied by #[deny(warnings)]
note: lint level defined here
   --> src\librustc\lib.rs:23:9
    |
23  | #![deny(warnings)]
    |         ^^^^^^^^
error: aborting due to previous error
error: Could not compile `rustc`.
Build failed, waiting for other jobs to finish...
error: build failed
