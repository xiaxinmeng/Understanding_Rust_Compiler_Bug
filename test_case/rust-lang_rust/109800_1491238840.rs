plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
diff of stderr:

2   --> $DIR/transmute-padding-ice.rs:27:40
3    |
4 LL |     assert::is_maybe_transmutable::<B, A>();
-    |                                        ^ `B` cannot be safely transmuted into `A` in the defining scope of `assert::Context`.
+    |                                        |
+    |                                        |
+    |                                        `B` cannot be safely transmuted into `A` in the defining scope of `assert::Context`.
+    |                                        Reason: DstIsTooBig
6    |
7    = help: the trait `BikeshedIntrinsicFrom<B, assert::Context, Assume { alignment: true, lifetimes: true, safety: true, validity: true }>` is not implemented for `A`
8 note: required by a bound in `is_maybe_transmutable`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/transmute-padding-ice/transmute-padding-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmute/transmute-padding-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/transmute/transmute-padding-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/transmute-padding-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/transmute-padding-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `B` cannot be safely transmuted into `A` in the defining scope of `assert::Context`.
  --> fake-test-src-base/transmute/transmute-padding-ice.rs:27:40
   |
LL |     assert::is_maybe_transmutable::<B, A>();
   |                                        |
   |                                        |
   |                                        `B` cannot be safely transmuted into `A` in the defining scope of `assert::Context`.
   |                                        Reason: DstIsTooBig
   |
   = help: the trait `BikeshedIntrinsicFrom<B, assert::Context, Assume { alignment: true, lifetimes: true, safety: true, validity: true }>` is not implemented for `A`
note: required by a bound in `is_maybe_transmutable`
  --> fake-test-src-base/transmute/transmute-padding-ice.rs:11:14
   |
LL |       pub fn is_maybe_transmutable<Src, Dst>()
LL |       where
LL |       where
LL |           Dst: BikeshedIntrinsicFrom<
LL | |             Src,
LL | |             Context,
LL | |             Context,
LL | |             { Assume { alignment: true, lifetimes: true, safety: true, validity: true } },
LL | |         >,
   | |_________^ required by this bound in `is_maybe_transmutable`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
