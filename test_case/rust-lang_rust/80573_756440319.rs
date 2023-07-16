plain
   Compiling ignore v0.4.16
   Compiling merge_derive v0.1.0
   Compiling merge v0.1.0
   Compiling toml v0.5.7
error[E0599]: no method named `rustarg` found for struct `builder::Cargo` in the current scope
     |
     |
304  |                     cargo.rustarg("-Wrustc::internal");
     |                           ^^^^^^^ method not found in `builder::Cargo`
...
342  | tool_check_step!(Rustdoc, "src/tools/rustdoc", SourceType::InTree);
     | 
    ::: src/bootstrap/builder.rs:1545:1
     |
1545 | pub struct Cargo {
1545 | pub struct Cargo {
     | ---------------- method `rustarg` not found for this
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0599]: no method named `rustarg` found for struct `builder::Cargo` in the current scope
     |
     |
304  |                     cargo.rustarg("-Wrustc::internal");
     |                           ^^^^^^^ method not found in `builder::Cargo`
...
347  | tool_check_step!(Clippy, "src/tools/clippy", SourceType::InTree);
     | 
    ::: src/bootstrap/builder.rs:1545:1
     |
1545 | pub struct Cargo {
1545 | pub struct Cargo {
     | ---------------- method `rustarg` not found for this
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0599]: no method named `rustarg` found for struct `builder::Cargo` in the current scope
     |
     |
304  |                     cargo.rustarg("-Wrustc::internal");
     |                           ^^^^^^^ method not found in `builder::Cargo`
...
349  | tool_check_step!(Bootstrap, "src/bootstrap", SourceType::InTree);
     | 
    ::: src/bootstrap/builder.rs:1545:1
     |
1545 | pub struct Cargo {
1545 | pub struct Cargo {
     | ---------------- method `rustarg` not found for this
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors

