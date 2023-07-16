plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0405]: cannot find trait `Neg` in this scope
    |
510 | / macro_rules! nonzero_signed_operations {
510 | / macro_rules! nonzero_signed_operations {
511 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
512 | |         $(
513 | |             impl Neg for $Ty {
...   |
741 | |     }
742 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
    | |_- in this expansion of `nonzero_signed_operations!`
743 | 
744 | / nonzero_signed_operations! {
745 | |     NonZeroI8(i8) -> NonZeroU8(u8);
746 | |     NonZeroI16(i16) -> NonZeroU16(u16);
747 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
750 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation
    |
help: consider importing this trait
    |
