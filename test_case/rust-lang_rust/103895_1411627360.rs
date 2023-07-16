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
Successfully built d759004f0a86
Successfully tagged rust-ci:latest
Built container sha256:d759004f0a86240d7cbfa94065afac4b09adc7d59c936440cc30846a815934d5
Uploading finished image to https://ci-caches.rust-lang.org/docker/c542e110917c5b4fba28ff9dc7344d30d6fc265d79a93e82d1a6e3a016d7d09c3ef884d808fe7df42d7009837bd563107d058d3cbaa9c50fcfc764d5aadfac5c
upload failed: - to s3://rust-lang-ci-sccache2/docker/c542e110917c5b4fba28ff9dc7344d30d6fc265d79a93e82d1a6e3a016d7d09c3ef884d808fe7df42d7009837bd563107d058d3cbaa9c50fcfc764d5aadfac5c Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
---- [ui] tests/ui/check-cfg/well-known-values.rs stdout ----
diff of stderr:

6    |                   |
7    |                   help: did you mean: `"linux"`
8    |
-    = note: expected values for `target_os` are: aix, android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, nto, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vita, vxworks, wasi, watchos, windows, xous
+    = note: expected values for `target_os` are: aix, android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, nto, openbsd, psp, redox, solaris, solid_asp3, trusty, tvos, uefi, unknown, vita, vxworks, wasi, watchos, windows, xous
11 
12 warning: unexpected `cfg` condition value



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/well-known-values/well-known-values.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-cfg/well-known-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/check-cfg/well-known-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/well-known-values" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/well-known-values/auxiliary" "--check-cfg=values()" "-Z" "unstable-options"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/check-cfg/well-known-values.rs:7:7
   |
LL | #[cfg(target_os = "linuz")]
   |       ^^^^^^^^^^^^-------
   |       ^^^^^^^^^^^^-------
   |                   |
   |                   help: did you mean: `"linux"`
   |
   = note: expected values for `target_os` are: aix, android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, nto, openbsd, psp, redox, solaris, solid_asp3, trusty, tvos, uefi, unknown, vita, vxworks, wasi, watchos, windows, xous

warning: unexpected `cfg` condition value
  --> fake-test-src-base/check-cfg/well-known-values.rs:14:7
   |
   |
LL | #[cfg(target_has_atomic = "0")]
   |       ^^^^^^^^^^^^^^^^^^^^---
   |                           |
   |                           help: did you mean: `"8"`
   |
   = note: expected values for `target_has_atomic` are: 128, 16, 32, 64, 8, ptr
warning: unexpected `cfg` condition value
  --> fake-test-src-base/check-cfg/well-known-values.rs:21:7
   |
   |
LL | #[cfg(unix = "aa")]
   |           |
   |           help: remove the value
   |
   = note: no expected value for `unix`
   = note: no expected value for `unix`

warning: unexpected `cfg` condition value
  --> fake-test-src-base/check-cfg/well-known-values.rs:28:7
   |
LL | #[cfg(miri = "miri")]
   |           |
   |           help: remove the value
   |
   = note: no expected value for `miri`
   = note: no expected value for `miri`

warning: unexpected `cfg` condition value
  --> fake-test-src-base/check-cfg/well-known-values.rs:35:7
   |
LL | #[cfg(doc = "linux")]
   |       ^^^----------
   |          help: remove the value
   |
   = note: no expected value for `doc`

