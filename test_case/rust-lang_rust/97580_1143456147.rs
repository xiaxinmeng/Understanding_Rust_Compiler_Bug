plain
Successfully built 7a5f9b74c057
Successfully tagged rust-ci:latest
Built container sha256:7a5f9b74c057d0530128fc7788ff984222652704a0b2e6ce604570d6bfb8e10f
Uploading finished image to https://ci-caches.rust-lang.org/docker/6a9f2d15c383e5a8c3f65ed87cf17e97fb6e5a5a5df550e82f2c9b9f0bc64573d60ff8ccfa7fbd06fb3e3d891dab927e307eab2dcff6cc2e2c3d2993fe31d69b
upload failed: - to s3://rust-lang-ci-sccache2/docker/6a9f2d15c383e5a8c3f65ed87cf17e97fb6e5a5a5df550e82f2c9b9f0bc64573d60ff8ccfa7fbd06fb3e3d891dab927e307eab2dcff6cc2e2c3d2993fe31d69b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
failures:

---- [ui] src/test/ui/borrowck/issue-71546.rs stdout ----

error: `stderr` should not have different output from base test!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-71546.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-71546" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-71546/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
   |
   |
LL |       let csv_str: String = value //~ ERROR: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
LL | |         .into_iter()
LL | |         .into_iter()
LL | |         .map(|elem| elem.to_string())
   |
   |
   = help: consider adding an explicit lifetime bound `<&'a V as IntoIterator>::Item: 'static`...
   = note: ...so that the type `<&'a V as IntoIterator>::Item` will meet its required lifetime bounds...
note: ...that is required by this bound
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     for<'a> <&'a V as IntoIterator>::Item: ToString + 'static,

error: aborting due to previous error

For more information about this error, try `rustc --explain E0310`.
