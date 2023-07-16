plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0433]: failed to resolve: use of undeclared crate or module `cfg_if`
   --> library/core/src/ffi/mod.rs:101:5
    |
101 |     cfg_if::cfg_if! {
    |     ^^^^^^ use of undeclared crate or module `cfg_if`

error[E0412]: cannot find type `c_char` in module `c_char_definition`
   |
   |
49 | type_alias! { "c_char.md", c_char = c_char_definition::c_char, NonZero_c_char = c_char_definition::NonZero_c_char;
   |                                                        ^^^^^^ not found in `c_char_definition`

error[E0412]: cannot find type `NonZero_c_char` in module `c_char_definition`
   |
   |
49 | type_alias! { "c_char.md", c_char = c_char_definition::c_char, NonZero_c_char = c_char_definition::NonZero_c_char;
   |                                                                                                    ^^^^^^^^^^^^^^ not found in `c_char_definition`
error: unused macro definition: `cfg_if`
   --> library/core/src/internal_macros.rs:215:14
    |
215 | macro_rules! cfg_if {
215 | macro_rules! cfg_if {
    |              ^^^^^^
    |
    = note: `-D unused-macros` implied by `-D warnings`
Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `core` due to 4 previous errors
Build completed unsuccessfully in 0:01:16
