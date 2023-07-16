plain
................iii.iiiii........ii........i................................

failures:

---- [mir-opt] tests/mir-opt/building/async_await.rs stdout ----
4         _0: GeneratorSavedTy {
5             ty: impl std::future::Future<Output = ()>,
6             source_info: SourceInfo {
-                 span: $DIR/async_await.rs:15:9: 15:14 (#9),
+                 span: $DIR/async_await.rs:15:9: 15:14 (#8),
8                 scope: scope[0],
10             ignore_for_traits: false,


12         _1: GeneratorSavedTy {
13             ty: impl std::future::Future<Output = ()>,
14             source_info: SourceInfo {
-                 span: $DIR/async_await.rs:16:9: 16:14 (#11),
+                 span: $DIR/async_await.rs:16:9: 16:14 (#10),
16                 scope: scope[0],
18             ignore_for_traits: false,


thread '[mir-opt] tests/mir-opt/building/async_await.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/building/async_await.b-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3634:21


failures:
    [mir-opt] tests/mir-opt/building/async_await.rs
