
Building stage2 tool rls (x86_64-apple-darwin)
   Compiling rls v0.126.0 (file:///Users/alex/Software/rust/src/tools/rls)
error[E0432]: unresolved import `cargo::core::Profile`
  --> tools/rls/src/build/plan.rs:34:30
   |
34 | use cargo::core::{PackageId, Profile, Target, TargetKind};
   |                              ^^^^^^^ no `Profile` in `core`. Did you mean to use `profiles`?

error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: Could not compile `rls`.
