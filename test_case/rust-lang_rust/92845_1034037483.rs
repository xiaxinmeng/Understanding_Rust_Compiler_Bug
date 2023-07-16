plain
Successfully built e4b7705fd075
Successfully tagged rust-ci:latest
Built container sha256:e4b7705fd0752eff9170d87284fa2f72282bcd000308af16e30c0a3ebc63a494
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc-demangle v0.1.21
    Checking rustc-std-workspace-alloc v1.99.0 (/checkout/library/rustc-std-workspace-alloc)
    Checking panic_unwind v0.0.0 (/checkout/library/panic_unwind)
    Checking panic_abort v0.0.0 (/checkout/library/panic_abort)
error[E0432]: unresolved imports `real_imp::eh_frame_registry`, `real_imp`
   |
   |
58 |         pub use real_imp::eh_frame_registry::*;
   |                           ^^^^^^^^^^^^^^^^^ could not find `eh_frame_registry` in `real_imp`
...
83 |         use real_imp as imp;

    Checking gimli v0.25.0
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.12.0
