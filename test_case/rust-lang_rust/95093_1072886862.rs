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

---- [ui] ui/mismatched_types/overloaded-calls-bad.rs stdout ----
diff of stderr:

18 LL |     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
20 
+ error[E0308]: mismatched types
+   --> $DIR/overloaded-calls-bad.rs:31:17
+    |
+    |
+ LL |     let ans = s("burma", "shave");
+    |                 ^^^^^^^ expected `isize`, found `&str`
21 error[E0057]: this function takes 1 argument but 2 arguments were supplied
22   --> $DIR/overloaded-calls-bad.rs:31:15
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
23    |
23    |

32 LL |     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
34 
- error: aborting due to 3 previous errors
+ error: aborting due to 4 previous errors
36 
---
To only update this specific test, also pass `--test-args mismatched_types/overloaded-calls-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/overloaded-calls-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/overloaded-calls-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/overloaded-calls-bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/overloaded-calls-bad.rs:28:17
   |
LL |     let ans = s("what");    //~ ERROR mismatched types
   |                 ^^^^^^ expected `isize`, found `&str`
error[E0057]: this function takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/mismatched_types/overloaded-calls-bad.rs:29:15
   |
   |
LL |     let ans = s();
   |               ^-- supplied 0 arguments
   |               expected 1 argument
   |
note: associated function defined here
  --> /checkout/library/core/src/ops/function.rs:150:27
  --> /checkout/library/core/src/ops/function.rs:150:27
   |
LL |     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/overloaded-calls-bad.rs:31:17
   |
   |
LL |     let ans = s("burma", "shave");
   |                 ^^^^^^^ expected `isize`, found `&str`
error[E0057]: this function takes 1 argument but 2 arguments were supplied
  --> /checkout/src/test/ui/mismatched_types/overloaded-calls-bad.rs:31:15
   |
   |
LL |     let ans = s("burma", "shave");
   |               ^ -------  ------- supplied 2 arguments
   |               expected 1 argument
   |
note: associated function defined here
  --> /checkout/library/core/src/ops/function.rs:150:27
  --> /checkout/library/core/src/ops/function.rs:150:27
   |
LL |     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0057, E0308.
