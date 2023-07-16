plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: implementation has missing stability attribute
   --> library/core/src/num/nonzero.rs:69:13
    |
50  | / macro_rules! nonzero_integers {
51  | |     ( $( #[$stability: meta] #[$const_new_unchecked_stability: meta] $Ty: ident($Int: ty); )+ ) => {
52  | |         $(
53  | |             /// An integer that is known not to equal zero.
...   |
69  | |             impl Sealed for $Ty {}
...   |
185 | |     }
186 | | }
    | |_- in this expansion of `nonzero_integers!`
    | |_- in this expansion of `nonzero_integers!`
187 | 
188 | / nonzero_integers! {
189 | |     #[stable(feature = "nonzero", since = "1.28.0")] #[rustc_const_stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
190 | |     #[stable(feature = "nonzero", since = "1.28.0")] #[rustc_const_stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
191 | |     #[stable(feature = "nonzero", since = "1.28.0")] #[rustc_const_stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
...   |
200 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] #[rustc_const_stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIs...
    | |_- in this macro invocation

error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:00:10
