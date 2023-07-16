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
Successfully built b2545cd4da1d
Successfully tagged rust-ci:latest
Built container sha256:b2545cd4da1d35b1b2eb5898696797ab4c7a6956241a4630fb5de90083ffbd59
Uploading finished image to https://ci-caches.rust-lang.org/docker/c542e110917c5b4fba28ff9dc7344d30d6fc265d79a93e82d1a6e3a016d7d09c3ef884d808fe7df42d7009837bd563107d058d3cbaa9c50fcfc764d5aadfac5c
upload failed: - to s3://rust-lang-ci-sccache2/docker/c542e110917c5b4fba28ff9dc7344d30d6fc265d79a93e82d1a6e3a016d7d09c3ef884d808fe7df42d7009837bd563107d058d3cbaa9c50fcfc764d5aadfac5c Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---

---- [ui] tests/ui/suggestions/issue-71394-no-from-impl.rs stdout ----
diff of stderr:

5    |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
7    = help: the following other types implement trait `From<T>`:
7    = help: the following other types implement trait `From<T>`:
+              <&'input [u8] as From<gimli::read::endian_slice::EndianSlice<'input, Endian>>>
8              <[T; LANES] as From<Simd<T, LANES>>>
9              <[bool; LANES] as From<Mask<T, LANES>>>
10    = note: required for `&[u8]` to implement `Into<&[i8]>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl/issue-71394-no-from-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-71394-no-from-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/issue-71394-no-from-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&[i8]: From<&[u8]>` is not satisfied
  --> fake-test-src-base/suggestions/issue-71394-no-from-impl.rs:3:25
   |
LL |     let _: &[i8] = data.into();
   |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
   = help: the following other types implement trait `From<T>`:
   = help: the following other types implement trait `From<T>`:
             <&'input [u8] as From<gimli::read::endian_slice::EndianSlice<'input, Endian>>>
             <[T; LANES] as From<Simd<T, LANES>>>
             <[bool; LANES] as From<Mask<T, LANES>>>
   = note: required for `&[u8]` to implement `Into<&[i8]>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
