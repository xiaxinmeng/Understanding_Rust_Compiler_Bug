plain
116 |   macro_rules! format {
    |   ------------------- in this expansion of `format!`
...
119 |           res
    |           ^^^ expected enum `LinkRlibError`, found struct `std::string::String`
   ::: compiler/rustc_codegen_ssa/src/back/link.rs:225:28
    |
225 |                   return Err(format!(
    |  ____________________________-
    |  ____________________________-
226 | |                     "{ty1:?} and {ty2:?} do not have equivalent dependency formats (`{list1:?}` vs `{list2:?}`)"
    | |_________________- in this macro invocation

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_ssa` due to previous error
