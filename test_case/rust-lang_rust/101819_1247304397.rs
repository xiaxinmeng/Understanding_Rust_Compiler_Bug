plain
Successfully built 24466cae975a
Successfully tagged rust-ci:latest
Built container sha256:24466cae975af64876ac757dbf31ebf19f47edd8075ff919005aa2f8eab5b3a0
Uploading finished image to https://ci-caches.rust-lang.org/docker/100443de95ac5fd5d76f947e37204434fbf3cf3da99f888ec8bb4fbbceda7ed7d93b519998f8388225735159bb6cbee56e76ad72dc699a1cf736f3af809f03d0
upload failed: - to s3://rust-lang-ci-sccache2/docker/100443de95ac5fd5d76f947e37204434fbf3cf3da99f888ec8bb4fbbceda7ed7d93b519998f8388225735159bb6cbee56e76ad72dc699a1cf736f3af809f03d0 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
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
Build completed unsuccessfully in 0:03:21
