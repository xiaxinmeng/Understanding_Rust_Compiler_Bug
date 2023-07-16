plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
Step 7/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-13       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Running in 3f0fd176fa78
Removing intermediate container 3f0fd176fa78
 ---> 38598d98e4c2
Step 8/8 : ENV SCRIPT ../x.py --stage 2 test --exclude src/tools/tidy &&            ../x --stage 2 test tests/mir-opt                              --host='' --target=i686-unknown-linux-gnu &&            ../x.ps1 --stage 2 test tests/ui --pass=check                              --host='' --target=i686-unknown-linux-gnu &&            python3 ../x.py --stage 1 clippy -Awarnings &&            python2.7 ../x.py --stage 2 test src/tools/tidy
Removing intermediate container 0b783e6cd9d6
 ---> 65cc3a6cd5e3
Successfully built 65cc3a6cd5e3
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:65cc3a6cd5e3704efa006d25e25f919c4d37ceb6f8a2c91ec3ccdaa3040a5ae6
Uploading finished image to https://ci-caches.rust-lang.org/docker/bcf14fce040598f1a6b9d3e5198e0d88a2c00cfe6808e9b57b0d90dee3b8e6d0f95daa85f2a2050aeb73f3b9f0cb40f0c2cf491c83dfad8dac94a6b0a8a3f7cb
upload failed: - to s3://rust-lang-ci-sccache2/docker/bcf14fce040598f1a6b9d3e5198e0d88a2c00cfe6808e9b57b0d90dee3b8e6d0f95daa85f2a2050aeb73f3b9f0cb40f0c2cf491c83dfad8dac94a6b0a8a3f7cb Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
test result: ok. 14183 passed; 0 failed; 200 ignored; 0 measured; 0 filtered out; finished in 83.54s

 finished in 84.285 seconds
Build completed successfully in 0:01:26
+ python3 ../x.py --stage 1 clippy -Awarnings
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.27s
Copying stage0 library from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `(1, Some(1))`,
 right: `(2, Some(2))`: wrong number of generic parameters for DefId(2:7163 ~ core[cdb4]::iter::traits::iterator::Iterator): [&mut mir::traversal::Preorder<'_, '_>, fn((mir::BasicBlock, &mir::BasicBlockData<'_>)) {std::mem::drop::<(mir::BasicBlock, &mir::BasicBlockData<'_>)>}]', /checkout/compiler/rustc_middle/src/ty/context.rs:1762:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.69 (efbb2338 2023-02-01)
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
error: could not compile `rustc_middle`
Build completed successfully in 0:03:15
