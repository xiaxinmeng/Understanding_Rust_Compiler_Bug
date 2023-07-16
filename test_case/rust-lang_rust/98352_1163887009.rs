plain
[RUSTC-TIMING] rustc_attr test:false 4.321
[RUSTC-TIMING] rustc_session test:false 6.668
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_ast_passes test:false 6.398
error: to use a constant of type `NonNull` in a pattern, `NonNull` must be annotated with `#[derive(PartialEq, Eq)]`
    |
882 |                 DUMMY_SP,
    |                 ^^^^^^^^


error: to use a constant of type `NonNull` in a pattern, `NonNull` must be annotated with `#[derive(PartialEq, Eq)]`
    |
    |
903 |             ObjectSafetyViolation::AssocConst(name, DUMMY_SP) => {

error: unreachable pattern
   --> compiler/rustc_middle/src/traits/mod.rs:884:13
    |
    |
884 |             ObjectSafetyViolation::Method(name, MethodViolationCode::ReferencesSelfInput(_), _) => {
    |
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
error: unreachable pattern
   --> compiler/rustc_middle/src/traits/mod.rs:906:13
    |
    |
906 |             ObjectSafetyViolation::AssocConst(..) => "it contains this associated `const`".into(),

[RUSTC-TIMING] rustc_llvm test:false 0.301
[RUSTC-TIMING] rustc_expand test:false 12.108
[RUSTC-TIMING] rustc_middle test:false 16.585
