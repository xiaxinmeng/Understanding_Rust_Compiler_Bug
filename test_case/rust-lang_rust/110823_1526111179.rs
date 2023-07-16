plain
.............ii.iiii........ii.........i................................

failures:

---- [mir-opt] tests/mir-opt/building/async_await.rs stdout ----
12         _1: GeneratorSavedTy {
13             ty: impl std::future::Future<Output = ()>,
14             source_info: SourceInfo {
-                 span: $DIR/async_await.rs:16:8: 16:14 (#10),
+                 span: $DIR/async_await.rs:16:9: 16:14 (#10),
16                 scope: scope[0],
18             ignore_for_traits: false,


thread '[mir-opt] tests/mir-opt/building/async_await.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/building/async_await.b-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3553:21


failures:
    [mir-opt] tests/mir-opt/building/async_await.rs
