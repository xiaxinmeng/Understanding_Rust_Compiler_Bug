plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0433]: failed to resolve: use of undeclared crate or module `io`
   --> compiler/rustc_session/src/filesearch.rs:170:54
    |
170 |     fn default_from_current_exe() -> Result<PathBuf, io::Error> {
    |                                                      ^^ use of undeclared crate or module `io`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> compiler/rustc_session/src/filesearch.rs:173:29
    |
    |
173 |                 let mut p = canonicalize(exe)?;
    |
    = help: the trait `std::ops::Try` is not implemented for `std::path::PathBuf`

Some errors have detailed explanations: E0277, E0433.
