plain
Step 7/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-13       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Running in e848b1bf3b16
Removing intermediate container e848b1bf3b16
 ---> cccfaf2213c5
Step 8/8 : ENV SCRIPT ../x.py --stage 2 test --exclude src/tools/tidy &&            ../x --stage 2 test src/test/mir-opt                              --host='' --target=i686-unknown-linux-gnu &&            ../x.ps1 --stage 2 test src/test/ui --pass=check                              --host='' --target=i686-unknown-linux-gnu &&            python3 ../x.py --stage 1 clippy -Awarnings &&            python2.7 ../x.py --stage 2 test src/tools/tidy
Removing intermediate container 7d45c67a04ae
 ---> 4573dc0951ea
Successfully built 4573dc0951ea
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:4573dc0951ea52804ba9289469da949795fba90276be68b0e22ed08cdaa57ac3
Uploading finished image to https://ci-caches.rust-lang.org/docker/50c21f6e2f5ef4c86ad09dc6e8d3aa1311dd8436059f85515c315f744947a64d9fa6784b6a6482f78e422a00c9f99bf388e6923ca240a5dc5bf5409e818e2837
upload failed: - to s3://rust-lang-ci-sccache2/docker/50c21f6e2f5ef4c86ad09dc6e8d3aa1311dd8436059f85515c315f744947a64d9fa6784b6a6482f78e422a00c9f99bf388e6923ca240a5dc5bf5409e818e2837 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
test result: ok. 13896 passed; 0 failed; 199 ignored; 0 measured; 0 filtered out; finished in 63.76s

 finished in 64.415 seconds
Build completed successfully in 0:01:06
+ python3 ../x.py --stage 1 clippy -Awarnings
    Finished dev [unoptimized] target(s) in 0.05s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.18s
Copying stage0 library from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `(1, Some(1))`,
 right: `(2, Some(2))`: wrong number of generic parameters for DefId(2:7152 ~ core[6a45]::iter::traits::iterator::Iterator): [&mut mir::traversal::Preorder<'_, '_>, fn((mir::BasicBlock, &mir::BasicBlockData<'_>)) {std::mem::drop::<(mir::BasicBlock, &mir::BasicBlockData<'_>)>}]', /checkout/compiler/rustc_middle/src/ty/context.rs:1918:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.68 (73cbf88c 2023-01-03)
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
error: could not compile `rustc_middle`
Build completed unsuccessfully in 0:02:44
