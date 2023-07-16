
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:760:13: Broken MIR: generator contains type Config in MIR, but typeck only knows about {NonCopy, ()} and []
  --> src/test/ui/async-await/non-trivial-drop.rs:12:5
   |
LL |     || {
   |     ^^
