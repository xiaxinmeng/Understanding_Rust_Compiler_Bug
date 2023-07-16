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

---- [ui] tests/ui/autoref-autoderef/issue-38940.rs stdout ----
diff of stderr:

+ error: reached the recursion limit finding the struct tail for `Bottom`
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "20"]`
+ 
1 error[E0055]: reached the recursion limit while auto-dereferencing `J`
2   --> $DIR/issue-38940.rs:49:22
2   --> $DIR/issue-38940.rs:49:22
3    |

17    = note: expected reference `&Bottom`
18               found reference `&Top`
- error: aborting due to 2 previous errors
+ error: aborting due to 3 previous errors
21 
22 Some errors have detailed explanations: E0055, E0308.
---
To only update this specific test, also pass `--test-args autoref-autoderef/issue-38940.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/autoref-autoderef/issue-38940.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoref-autoderef/issue-38940" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoref-autoderef/issue-38940/auxiliary" "-Zdeduplicate-diagnostics=yes"
stdout: none
--- stderr -------------------------------
error: reached the recursion limit finding the struct tail for `Bottom`
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "20"]`

error[E0055]: reached the recursion limit while auto-dereferencing `J`
  --> fake-test-src-base/autoref-autoderef/issue-38940.rs:49:22
  --> fake-test-src-base/autoref-autoderef/issue-38940.rs:49:22
   |
LL |     let x: &Bottom = &t;
   |                      ^^ deref recursion limit reached
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "20"]` attribute to your crate (`issue_38940`)
error[E0308]: mismatched types
  --> fake-test-src-base/autoref-autoderef/issue-38940.rs:49:22
   |
   |
LL |     let x: &Bottom = &t;
   |            -------   ^^ expected `&Bottom`, found `&Top`
   |            expected due to this
   |
   |
   = note: expected reference `&Bottom`
              found reference `&Top`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0055, E0308.
For more information about an error, try `rustc --explain E0055`.
For more information about an error, try `rustc --explain E0055`.
------------------------------------------


---- [ui] tests/ui/did_you_mean/recursion_limit_deref.rs stdout ----
diff of stderr:

+ error: reached the recursion limit finding the struct tail for `Bottom`
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "20"]`
+ 
1 error[E0055]: reached the recursion limit while auto-dereferencing `J`
2   --> $DIR/recursion_limit_deref.rs:51:22
2   --> $DIR/recursion_limit_deref.rs:51:22
3    |

17    = note: expected reference `&Bottom`
18               found reference `&Top`
- error: aborting due to 2 previous errors
+ error: aborting due to 3 previous errors
21 
22 Some errors have detailed explanations: E0055, E0308.
---
To only update this specific test, also pass `--test-args did_you_mean/recursion_limit_deref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/did_you_mean/recursion_limit_deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit_deref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit_deref/auxiliary" "-Zdeduplicate-diagnostics=yes"
stdout: none
--- stderr -------------------------------
error: reached the recursion limit finding the struct tail for `Bottom`
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "20"]`

error[E0055]: reached the recursion limit while auto-dereferencing `J`
  --> fake-test-src-base/did_you_mean/recursion_limit_deref.rs:51:22
  --> fake-test-src-base/did_you_mean/recursion_limit_deref.rs:51:22
   |
LL |     let x: &Bottom = &t; //~ ERROR mismatched types
   |                      ^^ deref recursion limit reached
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "20"]` attribute to your crate (`recursion_limit_deref`)
error[E0308]: mismatched types
  --> fake-test-src-base/did_you_mean/recursion_limit_deref.rs:51:22
   |
   |
LL |     let x: &Bottom = &t; //~ ERROR mismatched types
   |            -------   ^^ expected `&Bottom`, found `&Top`
   |            expected due to this
   |
   |
   = note: expected reference `&Bottom`
              found reference `&Top`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0055, E0308.
For more information about an error, try `rustc --explain E0055`.
