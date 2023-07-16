console
     Created binary (application) `repro` package
+ cargo +stable check --manifest-path repro/Cargo.toml
   Compiling repro v0.1.0 (/git/repro/repro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
+ cargo +beta check --manifest-path repro/Cargo.toml
   Compiling repro v0.1.0 (/git/repro/repro)
error: proc-macro derive panicked
 --> src/main.rs:2:10
  |
2 | #[derive(repro::D)]
  |          ^^^^^^^^
  |
  = help: message: assertion failed: `(left == right)`
            left: `"pub enum E { V = 0x0, }"`,
           right: `"pub enum E { V = 0, }"`

error: aborting due to previous error
