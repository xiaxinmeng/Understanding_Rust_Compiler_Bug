
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:956:13: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout
 --> y.rs:6:1
  |
6 | fn main() { m_used!(x); }
  | ^^^^^^^^^

thread 'rustc' panicked at 'Box<dyn Any>', /home/njn/dev/rust2/compiler/rustc_errors/src/lib.rs:1289:9
