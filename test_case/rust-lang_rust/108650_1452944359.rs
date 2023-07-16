plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Successfully built 1a4e86b77eb4
Successfully tagged rust-ci:latest
Built container sha256:1a4e86b77eb4d273114a7fec4cee1862970c8aef0cd22ab0d3cd26294422753f
Uploading finished image to https://ci-caches.rust-lang.org/docker/2d8760e15cd716e46ea98fab8e7af60c712922f5df298e54391d8a5a58bcaa16a7d6f2f012a2cb04e2fd5f51945d9112e87fb2970f7c4b52f59f1373bcbb7693
upload failed: - to s3://rust-lang-ci-sccache2/docker/2d8760e15cd716e46ea98fab8e7af60c712922f5df298e54391d8a5a58bcaa16a7d6f2f012a2cb04e2fd5f51945d9112e87fb2970f7c4b52f59f1373bcbb7693 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-14]
---
failures:

---- [ui] tests/ui/coercion/issue-73886.rs stdout ----

error: /checkout/tests/ui/coercion/issue-73886.rs:2: expected error not found: non-primitive cast: `&&[i32; 1]` as `&[_]`
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/coercion/issue-73886.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/issue-73886" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/issue-73886/auxiliary"
    Error {
        line_num: 2,
        kind: Some(
            Error,
            Error,
        ),
        msg: "non-primitive cast: `&&[i32; 1]` as `&[_]`",
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
]

thread '[ui] tests/ui/coercion/issue-73886.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1421:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
