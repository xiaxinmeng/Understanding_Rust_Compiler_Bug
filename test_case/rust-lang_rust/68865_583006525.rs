
+diff --git a/src/librustc_target/spec/mipsel_unknown_linux_uclibc.rs b/src/librustc_target/spec/mipsel_unknown_linux_uclibc.rs
+index a8152011efa..1e730ea9d47 100644
+--- a/src/librustc_target/spec/mipsel_unknown_linux_uclibc.rs
++++ b/src/librustc_target/spec/mipsel_unknown_linux_uclibc.rs
+@@ -14,8 +14,8 @@ pub fn target() -> TargetResult {
+         linker_flavor: LinkerFlavor::Gcc,
+ 
+         options: TargetOptions {
+-            cpu: "mips32r2".to_string(),
+-            features: "+mips32r2,+soft-float".to_string(),
++            cpu: "mips32".to_string(),
++            features: "mips32,-mips32r2,+single-float".to_string(),
+             max_atomic_width: Some(32),
+             target_mcount: "_mcount".to_string(),
+ 
+-- 
+2.20.1
+
-- 
2.20.1
