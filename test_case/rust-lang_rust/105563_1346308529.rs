plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: x86_64-gnu-llvm-13
##[endgroup]
fatal: unknown commit origin/master
All commits in `HEAD` are present in `master`
fatal: unknown commit origin/beta
All commits in `HEAD` are present in `beta`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
Successfully built 693a851c43db
Successfully tagged rust-ci:latest
Built container sha256:693a851c43dba7f4e8767451adfe5e75a4d95761f100793b501353bb4cf0fc88
Uploading finished image to https://ci-caches.rust-lang.org/docker/b1a0a85c4f3e0152481e0b71dbe737e7b4fb0c60dbcf063ff9da8019873d4c93e975303ae5e36dd30955a39ae6bfa3cd2cb6916ab3ce031d0a6543c015771e73
upload failed: - to s3://rust-lang-ci-sccache2/docker/b1a0a85c4f3e0152481e0b71dbe737e7b4fb0c60dbcf063ff9da8019873d4c93e975303ae5e36dd30955a39ae6bfa3cd2cb6916ab3ce031d0a6543c015771e73 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---

---- [ui] src/test/ui/issues/issue-23122-2.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `<<<<<<<... as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next: Sized`
+ error[E0275]: overflow evaluating the requirement `<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next: Sized`
3    |
3    |
4 LL |     type Next = <GetNext<T::Next> as Next>::Next;
5    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_23122_2`)
- note: required for `GetNext<<<<<<... as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next>` to implement `Next`
+ note: required for `GetNext<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next>` to implement `Next`
10    |
10    |
11 LL | impl<T: Next> Next for GetNext<T> {
12    |               ^^^^     ^^^^^^^^^^
12    |               ^^^^     ^^^^^^^^^^
-    = note: the full type name has been written to '$TEST_BUILD_DIR/issues/issue-23122-2/issue-23122-2.long-type-hash.txt'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
15 error: aborting due to previous error
16 

---
To only update this specific test, also pass `--test-args issues/issue-23122-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23122-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next: Sized`
   |
   |
LL |     type Next = <GetNext<T::Next> as Next>::Next;
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_23122_2`)
note: required for `GetNext<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next>` to implement `Next`
   |
   |
LL | impl<T: Next> Next for GetNext<T> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
