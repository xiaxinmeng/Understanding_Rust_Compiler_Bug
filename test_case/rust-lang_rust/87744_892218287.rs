plain
Successfully built c803ff0d7dd5
Successfully tagged rust-ci:latest
Built container sha256:c803ff0d7dd5f4b2fb1bb0c03124b499cd8bc0e21582a03e5d61ca62bde78358
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8522bdbde0170e6b8638c2ee7936d2221f0e051b4634ed61ad299a1775319b857fabeb7dc5d6fa62739b362a15aa7c74c4f64f6f12f1718e9d3e82ccdcf01ba
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8522bdbde0170e6b8638c2ee7936d2221f0e051b4634ed61ad299a1775319b857fabeb7dc5d6fa62739b362a15aa7c74c4f64f6f12f1718e9d3e82ccdcf01ba Unable to locate credentials
 * branch              master     -> FETCH_HEAD
[CI_JOB_NAME=x86_64-gnu-llvm-10]
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
   Compiling ansi_term v0.11.0
   Compiling difference v2.0.0
   Compiling pretty_assertions v0.6.1
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0063]: missing field `force_rerun` in initializer of `flags::Subcommand`
   --> src/bootstrap/builder/tests.rs:482:22
    |
482 |         config.cmd = Subcommand::Test {
    |                      ^^^^^^^^^^^^^^^^ missing `force_rerun`

error[E0063]: missing field `force_rerun` in initializer of `flags::Subcommand`
   --> src/bootstrap/builder/tests.rs:523:22
    |
523 |         config.cmd = Subcommand::Test {
    |                      ^^^^^^^^^^^^^^^^ missing `force_rerun`

error[E0063]: missing field `force_rerun` in initializer of `flags::Subcommand`
   --> src/bootstrap/builder/tests.rs:579:22
    |
579 |         config.cmd = Subcommand::Test {
    |                      ^^^^^^^^^^^^^^^^ missing `force_rerun`
For more information about this error, try `rustc --explain E0063`.
error: could not compile `bootstrap` due to 3 previous errors


