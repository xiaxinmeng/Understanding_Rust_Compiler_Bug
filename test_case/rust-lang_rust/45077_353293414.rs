
     if build.config.llvm_static_stdcpp &&
        !target.contains("windows") &&
-       !target.contains("apple") {
+       !target.contains("apple") &&
+       !target.contains("freebsd") {
         cargo.env("LLVM_STATIC_STDCPP",
                   compiler_file(build.cxx(target).unwrap(), "libstdc++.a"));
-    }
+    } else if target.contains("freebsd") {
+        cargo.env("LLVM_STATIC_STDCPP",
+                  compiler_file(build.cxx(target).unwrap(), "libc++.a"));
+    }  
