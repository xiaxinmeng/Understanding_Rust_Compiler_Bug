plain
Successfully built 0a1e7cade3f1
Successfully tagged rust-ci:latest
Built container sha256:0a1e7cade3f14c138e931ea0918f38a7a9d029109d19679fec5bb35ad5b6db5f
Uploading finished image to https://ci-caches.rust-lang.org/docker/6a9f2d15c383e5a8c3f65ed87cf17e97fb6e5a5a5df550e82f2c9b9f0bc64573d60ff8ccfa7fbd06fb3e3d891dab927e307eab2dcff6cc2e2c3d2993fe31d69b
upload failed: - to s3://rust-lang-ci-sccache2/docker/6a9f2d15c383e5a8c3f65ed87cf17e97fb6e5a5a5df550e82f2c9b9f0bc64573d60ff8ccfa7fbd06fb3e3d891dab927e307eab2dcff6cc2e2c3d2993fe31d69b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---

---- [ui] src/test/rustdoc-ui/display-output.rs stdout ----
diff of stdout:

24 LL | fn foo(x: &dyn std::fmt::Display) {}
25    |        ^ help: if this is intentional, prefix it with an underscore: `_x`
- warning: function is never used: `foo`
+ warning: function `foo` is never used
28   --> $DIR/display-output.rs:13:4
29    |
29    |
30 LL | fn foo(x: &dyn std::fmt::Display) {}

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/display-output/display-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args display-output.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/display-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/display-output" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--test" "--test-args=--show-output" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/display-output/auxiliary"
running 1 test
test /checkout/src/test/rustdoc-ui/display-output.rs - foo (line 9) ... ok

successes:
---

warning: unused variable: `x`
  --> /checkout/src/test/rustdoc-ui/display-output.rs:13:8
   |
LL | fn foo(x: &dyn std::fmt::Display) {}
   |        ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: function `foo` is never used
  --> /checkout/src/test/rustdoc-ui/display-output.rs:13:4
   |
   |
LL | fn foo(x: &dyn std::fmt::Display) {}
   |
   = note: `#[warn(dead_code)]` implied by `#[warn(unused)]`

warning: 3 warnings emitted
