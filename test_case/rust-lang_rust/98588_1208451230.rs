plain
Successfully built 8223f4f8a44a
Successfully tagged rust-ci:latest
Built container sha256:8223f4f8a44a26bd45e89df6ab6f8ee48fd7ce1e1b32c872561ad18c53aa94b7
Uploading finished image to https://ci-caches.rust-lang.org/docker/55e0616340de9767316ac8a2060e4b2402700344c201ac04865af7e91102302ab3c0bf7f297a771d5ca396e00e9848a4e4279034a0339b65793190c46eafc724
upload failed: - to s3://rust-lang-ci-sccache2/docker/55e0616340de9767316ac8a2060e4b2402700344c201ac04865af7e91102302ab3c0bf7f297a771d5ca396e00e9848a4e4279034a0339b65793190c46eafc724 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
.....
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
83           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
84           _10 = &_1;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
85           StorageLive(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-          _28 = const _;                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _28 = const _;                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
87                                            // mir::Constant
88                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
89                                            // + literal: Const { ty: &i32, val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3513:25


failures:
    [mir-opt] src/test/mir-opt/issue-73223.rs
