plain
Successfully built e47a01bfa1b9
Successfully tagged rust-ci:latest
Built container sha256:e47a01bfa1b9607c41d143dc226822538a71f15fcf86f58c5f5461a3a008c1a1
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0615]: attempted to take value of method `did` on type `&AdtDef<'_>`
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:772:31
    |
772 |             .get_if_local(def.did)
    |
help: use parentheses to call the method
    |
    |
772 |             .get_if_local(def.did())


error[E0615]: attempted to take value of method `did` on type `&AdtDef<'_>`
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:775:52
    |
775 |             .unwrap_or_else(|| cx.tcx.def_span(def.did));
    |
help: use parentheses to call the method
    |
    |
775 |             .unwrap_or_else(|| cx.tcx.def_span(def.did()));

For more information about this error, try `rustc --explain E0615`.
error: could not compile `rustc_mir_build` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
