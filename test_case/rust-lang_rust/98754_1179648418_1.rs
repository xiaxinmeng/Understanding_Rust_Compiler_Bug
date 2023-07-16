
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:760:13: Broken MIR: generator contains type Client2 in MIR, but typeck only knows about {()} and []
  --> /home/jnelson/rust-lang/rust2/src/test/ui/generator/derived-drop-parent-expr.rs:25:13
   |
LL |     let f = move || match drop(Client2 { ..Client2::default() }) {
   |             ^^^^^^^
