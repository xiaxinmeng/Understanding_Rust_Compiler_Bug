plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: cannot find macro `doc_comment` in this scope
   --> library/core/src/num/nonzero.rs:209:17
    |
181 | / macro_rules! nonzero_leading_trailing_zeros {
182 | |     ( $( $Ty: ident($Uint: ty) , $LeadingTestExpr:expr ;)+ ) => {
183 | |         $(
184 | |             impl $Ty {
209 | |                 doc_comment! {
    | |                 ^^^^^^^^^^^
...   |
236 | |     }
236 | |     }
237 | | }
    | |_- in this expansion of `nonzero_leading_trailing_zeros!`
238 | 
239 | / nonzero_leading_trailing_zeros! {
240 | |     NonZeroU8(u8), u8::MAX;
241 | |     NonZeroU16(u16), u16::MAX;
242 | |     NonZeroU32(u32), u32::MAX;
...   |
251 | |     NonZeroIsize(usize), -1isize;
    | |_- in this macro invocation

error: cannot find macro `doc_comment` in this scope
   --> library/core/src/num/nonzero.rs:185:17
   --> library/core/src/num/nonzero.rs:185:17
    |
181 | / macro_rules! nonzero_leading_trailing_zeros {
182 | |     ( $( $Ty: ident($Uint: ty) , $LeadingTestExpr:expr ;)+ ) => {
183 | |         $(
184 | |             impl $Ty {
185 | |                 doc_comment! {
...   |
236 | |     }
237 | | }
237 | | }
    | |_- in this expansion of `nonzero_leading_trailing_zeros!`
238 | 
239 | / nonzero_leading_trailing_zeros! {
240 | |     NonZeroU8(u8), u8::MAX;
241 | |     NonZeroU16(u16), u16::MAX;
242 | |     NonZeroU32(u32), u32::MAX;
...   |
251 | |     NonZeroIsize(usize), -1isize;
    | |_- in this macro invocation

error: unused import: `crate::intrinsics`
 --> library/core/src/num/nonzero.rs:9:5
 --> library/core/src/num/nonzero.rs:9:5
  |
9 | use crate::intrinsics;
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to 3 previous errors

error: could not compile `core`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:13
