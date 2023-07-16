plain
Successfully built 7f6aa32cae86
Successfully tagged rust-ci:latest
Built container sha256:7f6aa32cae8617420228d41955b652d6aae8639f26280d894c92289b29d6c331
Uploading finished image to https://ci-caches.rust-lang.org/docker/55e0616340de9767316ac8a2060e4b2402700344c201ac04865af7e91102302ab3c0bf7f297a771d5ca396e00e9848a4e4279034a0339b65793190c46eafc724
upload failed: - to s3://rust-lang-ci-sccache2/docker/55e0616340de9767316ac8a2060e4b2402700344c201ac04865af7e91102302ab3c0bf7f297a771d5ca396e00e9848a4e4279034a0339b65793190c46eafc724 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
..............................iii....................................................... 13288/13337
.................................................
failures:

---- [ui] src/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs stdout ----

error: /checkout/src/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs:24: unexpected error: '24:31: 24:33: `x` does not live long enough [E0597]'

error: /checkout/src/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs:24: unexpected error: '24:36: 24:38: `x` does not live long enough [E0597]'

error: /checkout/src/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs:24: unexpected error: '24:36: 24:38: temporary value dropped while borrowed [E0716]'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 3 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty/auxiliary"
    Error {
        line_num: 24,
        kind: Some(
            Error,
            Error,
        ),
        msg: "24:31: 24:33: `x` does not live long enough [E0597]",
    Error {
        line_num: 24,
        kind: Some(
            Error,
            Error,
        ),
        msg: "24:36: 24:38: `x` does not live long enough [E0597]",
    Error {
        line_num: 24,
        kind: Some(
            Error,
            Error,
        ),
        msg: "24:36: 24:38: temporary value dropped while borrowed [E0716]",
    },
]

thread '[ui] src/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


failures:
failures:
    [ui] src/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs
test result: FAILED. 13219 passed; 1 failed; 117 ignored; 0 measured; 0 filtered out; finished in 101.39s

 finished in 101.910 seconds
Build completed unsuccessfully in 0:10:41
