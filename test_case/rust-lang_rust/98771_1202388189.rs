rust
--- a/compiler/rustc_target/src/spec/tests/tests_impl.rs
+++ b/compiler/rustc_target/src/spec/tests/tests_impl.rs
@@ -46,7 +46,10 @@ fn check_consistency(&self) {
                         )
                     }
                     (LinkerFlavor::Gcc, LldFlavor::Ld64) => {
-                        assert_matches!(flavor, LinkerFlavor::Gcc)
+                        assert_matches!(
+                            flavor,
+                            LinkerFlavor::Lld(LldFlavor::Ld64) | LinkerFlavor::Gcc
+                        )
                     }
                     (LinkerFlavor::Msvc | LinkerFlavor::Lld(LldFlavor::Link), LldFlavor::Link) => {
                         assert_matches!(
