plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0027]: pattern does not mention field `flag`
  --> src/discriminant.rs:45:27
   |
45 |             tag_encoding: TagEncoding::Niche { untagged_variant, ref niche_variants, niche_start },
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `flag`
help: include the missing field in the pattern
   |
   |
45 |             tag_encoding: TagEncoding::Niche { untagged_variant, ref niche_variants, niche_start, flag },
   |                                                                                                 ~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
   |
45 |             tag_encoding: TagEncoding::Niche { untagged_variant, ref niche_variants, niche_start, .. },

error[E0027]: pattern does not mention field `flag`
   --> src/discriminant.rs:116:9
    |
    |
116 |         TagEncoding::Niche { untagged_variant, ref niche_variants, niche_start } => {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `flag`
help: include the missing field in the pattern
    |
    |
116 |         TagEncoding::Niche { untagged_variant, ref niche_variants, niche_start, flag } => {
    |                                                                               ~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
    |
116 |         TagEncoding::Niche { untagged_variant, ref niche_variants, niche_start, .. } => {

For more information about this error, try `rustc --explain E0027`.
error: could not compile `rustc_codegen_cranelift` due to 2 previous errors
Build completed unsuccessfully in 0:02:49
