 patch
diff --git a/configure b/configure
index 60d3661..40db978 100755
--- a/configure
+++ b/configure
@@ -1324,7 +1323,8 @@ do
     make_dir $t/rt/compiler-rt
     for i in                                          \
       isaac sync test \
-      arch/i386 arch/x86_64 arch/arm arch/aarch64 arch/mips arch/powerpc
+      arch/x86_64
+      #arch/i386 arch/x86_64 arch/arm arch/aarch64 arch/mips arch/powerpc
     do
       make_dir $t/rt/stage$s/$i
     done
@@ -1551,7 +1551,8 @@ do

         msg "configuring LLVM for $gnu_t"

-        LLVM_TARGETS="--enable-targets=x86,x86_64,arm,aarch64,mips,powerpc"
+        #LLVM_TARGETS="--enable-targets=x86,x86_64,arm,aarch64,mips,powerpc"
+        LLVM_TARGETS="--enable-targets=x86_64"
         LLVM_BUILD="--build=$gnu_t"
         LLVM_HOST="--host=$gnu_t"
         LLVM_TARGET="--target=$gnu_t"
diff --git a/mk/llvm.mk b/mk/llvm.mk
index 1cbf4a9..9b89782 100644
--- a/mk/llvm.mk
+++ b/mk/llvm.mk
@@ -50,7 +50,7 @@ $$(LLVM_CONFIG_$(1)): $$(LLVM_DEPS) $$(LLVM_STAMP_$(1))
    $$(Q)touch $$(LLVM_CONFIG_$(1))

 clean-llvm$(1):
-   $$(Q)$$(MAKE) -C $$(CFG_LLVM_BUILD_DIR_$(1)) clean
+   $$(Q)$$(MAKE) -j4 -C $$(CFG_LLVM_BUILD_DIR_$(1)) clean

 endif

diff --git a/mk/main.mk b/mk/main.mk
index 1cd491a..463c21b 100644
--- a/mk/main.mk
+++ b/mk/main.mk
@@ -277,7 +277,8 @@ endif
 ######################################################################

 # FIXME: x86-ism
-LLVM_COMPONENTS=x86 arm aarch64 mips powerpc ipo bitreader bitwriter linker asmparser mcjit \
+#LLVM_COMPONENTS=x86 arm aarch64 mips powerpc ipo bitreader bitwriter linker asmparser mcjit interpreter instrumentation
+LLVM_COMPONENTS=x86 ipo bitreader bitwriter linker asmparser mcjit \
                 interpreter instrumentation

 # Only build these LLVM tools
diff --git a/mk/llvm.mk b/mk/llvm.mk
index 1cbf4a9..f8d5387 100644
--- a/mk/llvm.mk
+++ b/mk/llvm.mk
@@ -46,11 +46,11 @@ else

 $$(LLVM_CONFIG_$(1)): $$(LLVM_DEPS) $$(LLVM_STAMP_$(1))
    @$$(call E, make: llvm)
-   $$(Q)$$(MAKE) -C $$(CFG_LLVM_BUILD_DIR_$(1)) $$(CFG_LLVM_BUILD_ENV_$(1)) ONLY_TOOLS="$$(LLVM_TOOLS)"
+   $$(Q)$$(MAKE) -j4 -C $$(CFG_LLVM_BUILD_DIR_$(1)) $$(CFG_LLVM_BUILD_ENV_$(1)) ONLY_TOOLS="$$(LLVM_TOOLS)"
    $$(Q)touch $$(LLVM_CONFIG_$(1))

 clean-llvm$(1):
-   $$(Q)$$(MAKE) -C $$(CFG_LLVM_BUILD_DIR_$(1)) clean
+   $$(Q)$$(MAKE) -j4 -C $$(CFG_LLVM_BUILD_DIR_$(1)) clean

 endif

