plain
Successfully built 554151f27173
Successfully tagged rust-ci:latest
Built container sha256:554151f2717355cacf2e03b5ecb430540503a5fd2219c2cf172f574f39337afc
Uploading finished image to https://ci-caches.rust-lang.org/docker/b0b14de1f8ccdae9d6691fe51748b78fc0caa47e31fb6a1c51fdc5b51c020f4f6b285907dbf35a56f7233a5f40c7dfe408e7d8869346843744b7d1b2da0c8c4e
upload failed: - to s3://rust-lang-ci-sccache2/docker/b0b14de1f8ccdae9d6691fe51748b78fc0caa47e31fb6a1c51fdc5b51c020f4f6b285907dbf35a56f7233a5f40c7dfe408e7d8869346843744b7d1b2da0c8c4e Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-14]
---
---- [ui] tests/ui/lint/lint-non-snake-case-crate.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/lint-non-snake-case-crate.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate/auxiliary"
stdout: none
stderr: none

---- [ui] tests/ui/lint/lint-non-snake-case-crate-2.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/lint-non-snake-case-crate-2.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-non-snake-case-crate-2/auxiliary" "--crate-name" "NonSnakeCase"
stdout: none
stderr: none


failures:
    [ui] tests/ui/lint/lint-non-snake-case-crate.rs
