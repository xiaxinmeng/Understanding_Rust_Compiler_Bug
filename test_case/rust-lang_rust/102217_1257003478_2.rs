
PS C:\Users\jamie\test> cargo clean
PS C:\Users\jamie\test> cargo check      
   Compiling proc-macro2 v1.0.43
   Compiling quote v1.0.21
   Compiling unicode-ident v1.0.4
   Compiling syn v1.0.100
   Compiling pin-project-internal v1.0.8
    Checking pin-project v1.0.8
    Checking warn_test v0.1.0 (C:\Users\jamie\test)
warning: associated function `project_ref` is never used
 --> src\main.rs:7:1
  |
7 | #[pin_project(project = PinProjectTestProj)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `warn_test` (bin "warn_test") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 6.06s

PS C:\Users\jamie\test>
