plain
Successfully built 3939c3a00e42
Successfully tagged rust-ci:latest
Built container sha256:3939c3a00e426720b2fb245e21717f894cb8f8b52c3b8b9bffad883d45d3f9c4
Uploading finished image to https://ci-caches.rust-lang.org/docker/1c72b7d037d305d35e0735812f08df148b36d5d74ee11961d19c0c3224e3b4bbf04ff3ebce61c1c56645202b549f50aac19493c18cafdab069ba5b3de9c503bb
upload failed: - to s3://rust-lang-ci-sccache2/docker/1c72b7d037d305d35e0735812f08df148b36d5d74ee11961d19c0c3224e3b4bbf04ff3ebce61c1c56645202b549f50aac19493c18cafdab069ba5b3de9c503bb Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-9]
---
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.18
   Compiling panic_abort v0.0.0 (/checkout/library/panic_abort)
error: const-stable function cannot use `#[feature(const_fn)]`
    |
    |
511 | impl<T, A: Allocator> Vec<T, A> {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
749 |     #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
749 |     #[rustc_allow_const_fn_unstable(const_fn)]


error: `RawVec::<T, A>::capacity` is not yet stable as a const fn
    |
    |
750 |         self.buf.capacity()
    |
    = help: Const-stable functions can only call other const-stable functions


error: const-stable function cannot use `#[feature(const_fn)]`
    |
    |
511 | impl<T, A: Allocator> Vec<T, A> {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
1649|     #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
1649|     #[rustc_allow_const_fn_unstable(const_fn)]


error: const-stable function cannot use `#[feature(const_fn)]`
    |
    |
511 | impl<T, A: Allocator> Vec<T, A> {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
1665|     #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
1665|     #[rustc_allow_const_fn_unstable(const_fn)]

error: aborting due to 4 previous errors

error: could not compile `alloc`
