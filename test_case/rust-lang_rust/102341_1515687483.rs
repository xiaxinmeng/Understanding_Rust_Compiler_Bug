plain
   Compiling std v0.0.0 (/checkout/library/std)
[RUSTC-TIMING] build_script_build test:false 0.282
[RUSTC-TIMING] build_script_build test:false 0.296
[RUSTC-TIMING] build_script_build test:false 0.311
error: no rules expected the token `Neg`
    |
511 | / macro_rules! nonzero_signed_operations {
511 | / macro_rules! nonzero_signed_operations {
512 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
513 | |         $(
514 | |             impl $Ty {
...   |
883 | |             forward_ref_unop! { impl const Neg, neg for $Ty,
    | |                                            ^^^ no rules expected this token in macro call
886 | |     }
887 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
888 |
888 |
889 | / nonzero_signed_operations! {
890 | |     NonZeroI8(i8) -> NonZeroU8(u8);
891 | |     NonZeroI16(i16) -> NonZeroU16(u16);
892 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
895 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation
    |
   ::: library/core/src/internal_macros.rs:3:1
    |
    |
3   |   macro_rules! forward_ref_unop {
    |   ----------------------------- when calling this macro
    |
note: while trying to match `,`
   --> library/core/src/internal_macros.rs:4:21
    |
4   |     (impl $imp:ident, $method:ident for $t:ty) => {

[RUSTC-TIMING] cc test:false 0.664
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
[RUSTC-TIMING] build_script_build test:false 0.242
[RUSTC-TIMING] build_script_build test:false 0.413
error: const `impl` for trait `Neg` which is not marked with `#[const_trait]`
    |
511 | / macro_rules! nonzero_signed_operations {
511 | / macro_rules! nonzero_signed_operations {
512 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
513 | |         $(
514 | |             impl $Ty {
873 | |             impl const Neg for $Ty {
    | |                        ^^^
...   |
886 | |     }
886 | |     }
887 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
888 |
889 | / nonzero_signed_operations! {
890 | |     NonZeroI8(i8) -> NonZeroU8(u8);
891 | |     NonZeroI16(i16) -> NonZeroU16(u16);
892 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
895 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation
    |
   ::: library/core/src/ops/arith.rs:660:1
    |
    |
660 |   pub trait Neg {
    |   - help: mark `Neg` as const: `#[const_trait]`
    |
    = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
    = note: adding a non-const method body in the future would be a breaking change
[RUSTC-TIMING] core test:false 5.739
error: could not compile `core` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:18
