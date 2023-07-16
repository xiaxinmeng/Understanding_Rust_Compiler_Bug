plain
[TIMING] tool::Tidy { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
fmt check
tidy check
tidy: Skipping binary file check, read-only filesystem
##[error]tidy error: /checkout/compiler/rustc_codegen_ssa/src/back/linker.rs:1624: TODO is deprecated; use FIXME
##[error]tidy error: /checkout/compiler/rustc_mir_build/src/build/expr/as_rvalue.rs:571: \
Use a single space after dots in comments.
##[error]tidy error: /checkout/tests/ui/const-generics/generic_const_exprs/typeid-equality-by-subtyping.rs:20: trailing whitespace
some tidy checks failed
