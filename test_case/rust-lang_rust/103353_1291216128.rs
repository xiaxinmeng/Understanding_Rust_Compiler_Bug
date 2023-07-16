plain
Successfully built e6922c35fb88
Successfully tagged rust-ci:latest
Built container sha256:e6922c35fb88266efebc0e8034ebba713d6ce10f1ae87d6ca3ad55be39faced7
Uploading finished image to https://ci-caches.rust-lang.org/docker/5113f42bfd9b1c95a1d3883587ba8dad4c66787bb247c62a9135b39daac8eb518d79354d0685bc057d2ca9275608acb8acc0d283225531823eccf30a4e542d9b
upload failed: - to s3://rust-lang-ci-sccache2/docker/5113f42bfd9b1c95a1d3883587ba8dad4c66787bb247c62a9135b39daac8eb518d79354d0685bc057d2ca9275608acb8acc0d283225531823eccf30a4e542d9b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
iiii.i...................
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] src/test/codegen/issue-81408-dllimport-thinlto-windows.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/issue-81408-dllimport-thinlto-windows.rs" "-Zthreads=1" "-O" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-81408-dllimport-thinlto-windows" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target" "x86_64-pc-windows-msvc" "-C" "lto=thin" "-C" "linker-plugin-lto" "-C" "prefer-dynamic=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-81408-dllimport-thinlto-windows/auxiliary" "--emit=llvm-ir"
stdout: none
--- stderr -------------------------------
  |
  |
  = note: the `x86_64-pc-windows-msvc` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-pc-windows-msvc`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error[E0463]: can't find crate for `static_dllimport_aux`
error[E0463]: can't find crate for `static_dllimport_aux`
 --> /checkout/src/test/codegen/issue-81408-dllimport-thinlto-windows.rs:8:1
  |
8 | extern crate static_dllimport_aux;

error: requires `sized` lang_item

error: aborting due to 3 previous errors
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0463`.
------------------------------------------



failures:
    [codegen] src/test/codegen/issue-81408-dllimport-thinlto-windows.rs
test result: FAILED. 309 passed; 1 failed; 67 ignored; 0 measured; 0 filtered out; finished in 4.26s

Build completed unsuccessfully in 0:12:25
