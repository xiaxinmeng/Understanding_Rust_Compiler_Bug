plain
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_error_messages v0.0.0 (/checkout/compiler/rustc_error_messages)
error: no rules expected the token `)`
    |
19  | macro_rules! declare_features {
    | ----------------------------- when calling this macro
...
...
537 |     (incomplete, unsafe_cell_from_mut, None, None),
    |                                                  ^ no rules expected this token in macro call
    |
note: while trying to match `,`
   --> compiler/rustc_feature/src/active.rs:27:84
    |
27  |         $(#[doc = $doc:tt])* ($status:ident, $feature:ident, $ver:expr, $issue:expr, $edition:expr),

error[E0432]: unresolved import `crate::Features`
 --> compiler/rustc_feature/src/builtin_attrs.rs:7:13
  |
  |
7 | use crate::{Features, Stability};
  |             |
  |             no `Features` in the root
  |             help: a similar name exists in the module: `Feature`


error[E0432]: unresolved imports `active::Features`, `active::ACTIVE_FEATURES`
   --> compiler/rustc_feature/src/lib.rs:151:18
    |
151 | pub use active::{Features, ACTIVE_FEATURES, INCOMPATIBLE_FEATURES};
    |                  ^^^^^^^^  ^^^^^^^^^^^^^^^ no `ACTIVE_FEATURES` in `active`
    |                  no `Features` in `active`
    |                  help: a similar name exists in the module: `Feature`

error[E0412]: cannot find type `Features` in this scope
error[E0412]: cannot find type `Features` in this scope
   --> compiler/rustc_feature/src/active.rs:100:38
    |
100 |     pub fn set(&self, features: &mut Features, span: Span) {
    |                                      ^^^^^^^^ help: a struct with a similar name exists: `Feature`
   ::: compiler/rustc_feature/src/lib.rs:50:1
    |
50  | pub struct Feature {
    | ------------------ similarly named struct `Feature` defined here
    | ------------------ similarly named struct `Feature` defined here

error: unused import: `to_nonzero`
 --> compiler/rustc_feature/src/active.rs:3:13
  |
3 | use super::{to_nonzero, Feature, State};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused import: `rustc_data_structures::fx::FxHashSet`
  |
  |
5 | use rustc_data_structures::fx::FxHashSet;

error: unused import: `rustc_span::edition::Edition`
 --> compiler/rustc_feature/src/active.rs:6:5
  |
  |
6 | use rustc_span::edition::Edition;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `sym`
 --> compiler/rustc_feature/src/active.rs:7:26
  |
7 | use rustc_span::symbol::{sym, Symbol};

error: unused macro definition: `set`
  --> compiler/rustc_feature/src/active.rs:10:14
   |
   |
10 | macro_rules! set {
   |              ^^^
   |
   = note: `-D unused-macros` implied by `-D warnings`
Some errors have detailed explanations: E0412, E0432.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rustc_feature` due to 9 previous errors
warning: build failed, waiting for other jobs to finish...
