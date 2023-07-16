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
Removing intermediate container 3c62d9c83379
 ---> aa7021f1d5e2
Step 6/12 : RUN npm install eslint@8.6.0 -g
 ---> Running in aba0d766a0d3
---
Successfully built d8f00838b944
Successfully tagged rust-ci:latest
Built container sha256:d8f00838b94458c0465f3f1de81a3f49cb6bc83de90c2f90adcaa2da238f9ef5
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.59 (/checkout/src/tools/clippy/clippy_lints)
error[E0531]: cannot find tuple struct or tuple variant `LlvmInlineAsm` in this scope
    --> src/tools/clippy/clippy_lints/src/suspicious_operation_groupings.rs:570:12
     |
570  |         | (LlvmInlineAsm(_), LlvmInlineAsm(_))
     |            ^^^^^^^^^^^^^ help: a tuple variant with a similar name exists: `InlineAsm`
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1425:5
     |
     |
1425 |     InlineAsm(P<InlineAsm>),
     |     ----------------------- similarly named tuple variant `InlineAsm` defined here

error[E0531]: cannot find tuple struct or tuple variant `LlvmInlineAsm` in this scope
    --> src/tools/clippy/clippy_lints/src/suspicious_operation_groupings.rs:570:30
     |
570  |         | (LlvmInlineAsm(_), LlvmInlineAsm(_))
     |                              ^^^^^^^^^^^^^ help: a tuple variant with a similar name exists: `InlineAsm`
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1425:5
     |
     |
1425 |     InlineAsm(P<InlineAsm>),
     |     ----------------------- similarly named tuple variant `InlineAsm` defined here

error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
    |
    |
550 |             ExprKind::LlvmInlineAsm(_) => {
    |                       |
    |                       variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                       help: there is a variant with a similar name: `InlineAsm`


error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
    |
    |
307 |         hir::ExprKind::LlvmInlineAsm(asm) => {
    |                        |
    |                        variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                        help: there is a variant with a similar name: `InlineAsm`


error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
    |
    |
452 |         | ExprKind::LlvmInlineAsm(..)
    |                     |
    |                     variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                     help: there is a variant with a similar name: `InlineAsm`


error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
   --> src/tools/clippy/clippy_lints/src/entry.rs:507:52
    |
507 |                 ExprKind::InlineAsm(_) | ExprKind::LlvmInlineAsm(_) => {
    |                                                    |
    |                                                    variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                                                    help: there is a variant with a similar name: `InlineAsm`


error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `rustc_hir::ExprKind` in the current scope
   --> src/tools/clippy/clippy_lints/src/loops/never_loop.rs:184:21
    |
184 |         | ExprKind::LlvmInlineAsm(_)
    |                     |
    |                     variant or associated item not found in `rustc_hir::ExprKind<'_>`
    |                     help: there is a variant with a similar name: `InlineAsm`

