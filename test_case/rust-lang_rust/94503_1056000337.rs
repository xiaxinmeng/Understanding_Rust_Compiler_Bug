plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0433]: failed to resolve: could not find `cfg_if` in the crate root
   --> library/core/src/internal_macros.rs:223:17
    |
215 | / macro_rules! cfg_if {
216 | |     // match if/else chains with a final `else`
218 | |         $(
...   |
223 | |         $crate::cfg_if! {
    | |                 ^^^^^^ could not find `cfg_if` in the crate root
    | |                 ^^^^^^ could not find `cfg_if` in the crate root
...   |
280 | |     };
281 | | }
    | |_- in this expansion of `cfg_if!`
    |
   ::: library/core/src/ffi/mod.rs:101:5
    |
101 | /     cfg_if! {
102 | |         // These are the targets on which c_char is unsigned.
103 | |         if #[cfg(any(
104 | |             all(
151 | |         }
152 | |     }
    | |_____- in this macro invocation


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
Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `core` due to 3 previous errors
Build completed unsuccessfully in 0:01:23
