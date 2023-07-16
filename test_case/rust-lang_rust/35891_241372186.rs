 bash
failures:

---- _2 stdout ----
    error[E0412]: type name `Circle` is undefined or not in scope
 --> <anon>:8:18
  |
8 | impl HasArea for Circle {
  |                  ^^^^^^ undefined or not in scope
  |
  = help: no candidates by the name of `Circle` found in your project; maybe you misspelled the name or forgot to import an external crate?

error: aborting due to previous error(s)

thread '_2' panicked at 'Box<Any>', src/librustc/session/mod.rs:167
