plain
1 warning: unexpected `cfg` condition value
-   --> src/test/rustdoc-ui/check-cfg-test.rs:8:7
+   --> $DIR/check-cfg-test.rs:8:7
3    |
4 LL | #[cfg(feature = "invalid")]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-cfg-test/check-cfg-test.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-cfg-test/check-cfg-test.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-cfg-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/check-cfg-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-cfg-test" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "--nocapture" "--check-cfg=values(feature,\"test\")" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-cfg-test/auxiliary"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc-ui/check-cfg-test.rs - Foo (line 7) ... ok
---
------------------------------------------
warning: unexpected `cfg` condition value
  --> /checkout/src/test/rustdoc-ui/check-cfg-test.rs:8:7
   |
LL | #[cfg(feature = "invalid")]
   |
   = note: `#[warn(unexpected_cfgs)]` on by default

warning: 1 warning emitted
