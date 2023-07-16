plain
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
error[E0308]: mismatched types
  --> compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs:11:9
11 |         llvm_target,
   |         ^^^^^^^^^^^ expected enum `Cow`, found struct `std::string::String`
   |
   = note: expected enum `Cow<'static, str>`
