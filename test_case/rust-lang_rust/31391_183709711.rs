 diff
diff --git a/src/test/run-make/llvm-module-pass/Makefile b/src/test/run-make/llvm-module-pass/Makefile
index 339dd1c..0f457e7 100644
--- a/src/test/run-make/llvm-module-pass/Makefile
+++ b/src/test/run-make/llvm-module-pass/Makefile
@@ -10,5 +10,5 @@ all: $(call NATIVE_STATICLIB,llvm-pass)
    $(RUSTC) main.rs

 $(TMPDIR)/libllvm-pass.o:
-   $(CXX) $(LLVM_CXXFLAGS) -c llvm-pass.so.cc -o $(TMPDIR)/libllvm-pass.o
+   $(CXX) $(LLVM_CFLAGS) $(LLVM_CXXFLAGS) -c llvm-pass.so.cc -o $(TMPDIR)/libllvm-pass.o
 endif
