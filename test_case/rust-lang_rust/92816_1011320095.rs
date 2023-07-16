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
Removing intermediate container e3f1c0dc2a2e
 ---> ffdf6d7f49bc
Step 6/12 : RUN npm install eslint@8.6.0 -g
 ---> Running in 17671b7d7ef4
---
Successfully built cf71253fd535
Successfully tagged rust-ci:latest
Built container sha256:cf71253fd53564b04522ae4a324b51f22ecc47a8602f96976cdc659411995133
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking ignore v0.4.17
   Compiling structopt-derive v0.4.18
    Checking thiserror v1.0.30
    Checking structopt v0.3.25
error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `ExprKind` in the current scope
    |
    |
510 |         | ast::ExprKind::LlvmInlineAsm(..)
    |                          |
    |                          |
    |                          variant or associated item not found in `ExprKind`
    |                          help: there is a variant with a similar name: `InlineAsm`

error[E0599]: no variant or associated item named `LlvmInlineAsm` found for enum `ExprKind` in the current scope
    |
    |
337 |         ast::ExprKind::LlvmInlineAsm(..) | ast::ExprKind::InlineAsm(..) => {
    |                        |
    |                        |
    |                        variant or associated item not found in `ExprKind`
    |                        help: there is a variant with a similar name: `InlineAsm`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustfmt-nightly` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
