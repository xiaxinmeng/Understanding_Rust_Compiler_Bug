plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: x86_64-gnu-llvm-12
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-12
---

6    |     |
7    |     required by a bound introduced by this call
8    |
-    = help: the trait `Foo<'de>` is implemented for `Baz<T>`
10 note: required because of the requirements on the impl of `for<'de> Foo<'de>` for `Baz<EmptyBis<'de>>`
12    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-96223/issue-96223.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-96223.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-96223.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-96223" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-96223/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `for<'de> EmptyBis<'de>: Foo<'_>` is not satisfied
   |
   |
LL |     icey_bounds(&p); //~ERROR the trait bound
   |     ----------- ^^ the trait `for<'de> Foo<'_>` is not implemented for `EmptyBis<'de>`
   |     required by a bound introduced by this call
   |
   |
note: required because of the requirements on the impl of `for<'de> Foo<'de>` for `Baz<EmptyBis<'de>>`
   |
   |
LL | impl<'de, T> Foo<'de> for Baz<T> where T: Foo<'de> {}
   |              ^^^^^^^^     ^^^^^^
note: required because of the requirements on the impl of `Dummy<EmptyMarker>` for `Empty`
   |
   |
LL | impl<M> Dummy<M> for Empty
   |         ^^^^^^^^     ^^^^^
note: required by a bound in `icey_bounds`
   |
   |
LL | fn icey_bounds<D: Dummy<EmptyMarker>>(p: &D) {}
   |                   ^^^^^^^^^^^^^^^^^^ required by this bound in `icey_bounds`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
