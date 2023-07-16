diff
diff --git a/src/test/ui/abi/stack-probes-lto.rs b/src/test/ui/abi/stack-probes-lto.rs
index 90df1f3f53e..bc3f61248a1 100644
--- a/src/test/ui/abi/stack-probes-lto.rs
+++ b/src/test/ui/abi/stack-probes-lto.rs
@@ -12,7 +12,7 @@
 // ignore-sgx no processes
 // ignore-musl FIXME #31506
 // ignore-pretty
-// compile-flags: -C lto
+// compile-flags: -C lto -Ccodegen-units=4
 // no-prefer-dynamic

 include!("stack-probes.rs");
