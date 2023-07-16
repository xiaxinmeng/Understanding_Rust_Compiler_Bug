plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: expected expression, found `$`
   --> library/core/src/num/nonzero.rs:914:67
    |
899 | / macro_rules! nonzero_min_max {
900 | |     ( $( $MinVal:expr , $Ty: ident($Int: ty); )+ ) => {
901 | |         $(
902 | |             impl $Ty {
...   |
914 | |                 pub const MIN : $Ty = unsafe { $Ty::new_unchecked($Min)};
...   |
917 | |     }
918 | | }
918 | | }
    | |_- in this expansion of `nonzero_min_max!`
919 | 
920 | / nonzero_min_max! {
921 | |     1 , NonZeroU8(u8);
922 | |     1 , NonZeroU16(u16);
923 | |     1 , NonZeroU32(u32);
...   |
932 | |     isize::MIN  , NonZeroIsize(isize);
    | |_- in this macro invocation

error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:01:11
