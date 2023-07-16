plain

4 packages are looking for funding
  run `npm fund` for details

8 vulnerabilities (6 moderate, 2 high)
To address issues that do not require attention, run:
  npm audit fix


Some issues need review, and may require choosing

Run `npm audit` for details.
npm notice 
npm notice 
npm notice New major version of npm available! 7.21.1 -> 8.3.0
npm notice Changelog: <https://github.com/npm/cli/releases/tag/v8.3.0>
npm notice Run `npm install -g npm@8.3.0` to update!
Removing intermediate container 5f6b04541a3a
 ---> d180ed5bcb67
Step 6/12 : RUN npm install eslint@8.6.0 -g
 ---> Running in 591fbc0e8093
---
Successfully built 0e896cbd8719
Successfully tagged rust-ci:latest
Built container sha256:0e896cbd8719adbd22b46c998f83bea9b8bac2a3f17ddbabd12dc1364d773044
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
   Compiling clippy v0.1.59 (/checkout/src/tools/clippy)
    Checking bstr v0.2.13
    Checking quote v1.0.7
   Compiling libz-sys v1.1.3
error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
   --> src/tools/clippy/clippy_utils/src/eager_or_lazy.rs:178:29
    |
178 |                 | ExprKind::LlvmInlineAsm(_)
    |                             |
    |                             variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                             help: there is a variant with a similar name: `InlineAsm`


error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:688:23
    |
688 |             ExprKind::LlvmInlineAsm(..) | ExprKind::Err => {},
    |                       |
    |                       variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                       help: there is a variant with a similar name: `InlineAsm`


    Checking idna v0.2.0
error[E0599]: no variant named `LlvmInlineAsm` found for enum `StatementKind<'_>`
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:225:24
    |
225 |         StatementKind::LlvmInlineAsm { .. } => Err((span, "cannot use inline assembly in const fn".into())),

    Checking getrandom v0.2.0
    Checking dirs-sys-next v0.1.2
    Checking num_cpus v1.13.0
    Checking num_cpus v1.13.0
    Checking parking_lot_core v0.8.5
    Checking filetime v0.2.14
error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
   --> src/tools/clippy/clippy_utils/src/sugg.rs:150:30
    |
150 |             | hir::ExprKind::LlvmInlineAsm(..)
    |                              |
    |                              variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                              help: there is a variant with a similar name: `InlineAsm`


error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_ast::ExprKind` in the current scope
   --> src/tools/clippy/clippy_utils/src/sugg.rs:208:30
    |
208 |             | ast::ExprKind::LlvmInlineAsm(..)
    |                              |
    |                              variant or associated item not found in `rustc_ast::ExprKind`
    |                              help: there is a variant with a similar name: `InlineAsm`


    Checking dirs-next v2.0.0
error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
    |
    |
781 |         | ExprKind::LlvmInlineAsm(_) => false,
    |                     |
    |                     variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                     help: there is a variant with a similar name: `InlineAsm`

