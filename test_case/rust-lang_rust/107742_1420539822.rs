plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:fbd127f5fb2c74988e07e7abbc13faa754861242)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---

---- [ui] tests/ui/recursion_limit/zero-overflow.rs stdout ----
diff of stderr:

+ error: queries overflow the depth limit!
+    |
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "2"]` attribute to your crate (`zero_overflow`)
+    = note: query depth increased by 1 when computing layout of `()`
+ 
1 error[E0275]: overflow evaluating the requirement `&mut Self: DispatchFromDyn<&mut RustaceansAreAwesome>`
2    |
3    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "2"]` attribute to your crate (`zero_overflow`)
4 
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
6 
---
To only update this specific test, also pass `--test-args recursion_limit/zero-overflow.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/recursion_limit/zero-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion_limit/zero-overflow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion_limit/zero-overflow/auxiliary"
stdout: none
--- stderr -------------------------------
error: queries overflow the depth limit!
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "2"]` attribute to your crate (`zero_overflow`)
   = note: query depth increased by 1 when computing layout of `()`

error[E0275]: overflow evaluating the requirement `&mut Self: DispatchFromDyn<&mut RustaceansAreAwesome>`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "2"]` attribute to your crate (`zero_overflow`)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
