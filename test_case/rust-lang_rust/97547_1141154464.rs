plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0711]: feature `nonzero_checked_ops` is declared stable since 1.63, but was previously declared stable since 1.63.0
    |
513 | / macro_rules! nonzero_signed_operations {
513 | / macro_rules! nonzero_signed_operations {
514 | |     ( $( $Ty: ident($Int: ty) -> $Uty: ident($Uint: ty); )+ ) => {
515 | |         $(
516 | |             #[stable(feature = "nonzero_checked_ops", since = "1.63")]
...   |
752 | |     }
753 | | }
    | |_- in this expansion of `nonzero_signed_operations!`
    | |_- in this expansion of `nonzero_signed_operations!`
754 | 
755 | / nonzero_signed_operations! {
756 | |     NonZeroI8(i8) -> NonZeroU8(u8);
757 | |     NonZeroI16(i16) -> NonZeroU16(u16);
758 | |     NonZeroI32(i32) -> NonZeroU32(u32);
...   |
761 | |     NonZeroIsize(isize) -> NonZeroUsize(usize);
    | |_- in this macro invocation

error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:00:09
