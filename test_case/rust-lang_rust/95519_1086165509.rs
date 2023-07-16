plain
   Compiling thorin-dwp v0.2.0
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
   Compiling tracing-tree v0.2.0
error[E0309]: the parameter type `V` may not live long enough
    |
149 |     type IntoIter = impl Iterator<Item = Self::Item>;
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = help: consider adding an explicit lifetime bound `V: 'a`...
    = note: ...so that the type `(K, V)` will meet its required lifetime bounds...
note: ...that is required by this bound
    |
    |
187 | pub struct IterMut<'a, T: 'a> {


error[E0309]: the parameter type `K` may not live long enough
    |
149 |     type IntoIter = impl Iterator<Item = Self::Item>;
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = help: consider adding an explicit lifetime bound `K: 'a`...
    = note: ...so that the type `(K, V)` will meet its required lifetime bounds...
note: ...that is required by this bound
    |
    |
187 | pub struct IterMut<'a, T: 'a> {

   Compiling chalk-solve v0.80.0
   Compiling rustc_log v0.0.0 (/checkout/compiler/rustc_log)
For more information about this error, try `rustc --explain E0309`.
