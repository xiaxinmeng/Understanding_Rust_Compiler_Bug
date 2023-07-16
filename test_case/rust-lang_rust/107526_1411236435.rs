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
Successfully built eb53c03b3857
Successfully tagged rust-ci:latest
Built container sha256:eb53c03b385785bd9d5b0a57b52ed496ef906f7765e390cff136225035533ae6
Uploading finished image to https://ci-caches.rust-lang.org/docker/c542e110917c5b4fba28ff9dc7344d30d6fc265d79a93e82d1a6e3a016d7d09c3ef884d808fe7df42d7009837bd563107d058d3cbaa9c50fcfc764d5aadfac5c
upload failed: - to s3://rust-lang-ci-sccache2/docker/c542e110917c5b4fba28ff9dc7344d30d6fc265d79a93e82d1a6e3a016d7d09c3ef884d808fe7df42d7009837bd563107d058d3cbaa9c50fcfc764d5aadfac5c Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
---- [ui] tests/ui/mut/no-mut-lint-for-desugared-mut.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mut/no-mut-lint-for-desugared-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/no-mut-lint-for-desugared-mut/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/no-mut-lint-for-desugared-mut/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found block
  --> fake-test-src-base/mut/no-mut-lint-for-desugared-mut.rs:7:14
   |
LL |     for _ in { return (); 0..3 } {} // ok
   |
   |
   = help: try adding an expression to the `for` loop: `for PATTERN in EXPRESSION { BODY }`
error: aborting due to previous error
------------------------------------------


