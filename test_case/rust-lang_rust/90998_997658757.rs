plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing const stability attribute
   --> library/core/src/num/nonzero.rs:926:17
    |
903 |  / macro_rules! nonzero_unsigned_is_power_of_two {
904 |  |     ( $( $Ty: ident )+ ) => {
905 |  |         $(
906 |  |             impl $Ty {
...    |
926 | /|                 pub const fn is_power_of_two(self) -> bool {
927 | ||                     // LLVM 11 normalizes `unchecked_sub(x, 1) & x == 0` to the implementation seen here.
928 | ||                     // On the basic x86-64 target, this saves 3 instructions for the zero check.
929 | ||                     // On x86_64 with BMI1, being nonzero lets it codegen to `BLSR`, which saves an instruction
...   ||
932 | ||                     intrinsics::ctpop(self.get()) < 2
    | ||_________________^
...    |
937 |  |     }
938 |  | }
938 |  | }
    |  |_- in this expansion of `nonzero_unsigned_is_power_of_two!`
939 | 
940 |    nonzero_unsigned_is_power_of_two! { NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize }

error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:04:50
