diff
--- config.toml.example 2020-09-14 11:55:08.330000000 +0200
+++ config.toml 2020-09-14 11:55:08.250000000 +0200
@@ -48,7 +48,7 @@
 #release-debuginfo = false

 # Indicates whether the LLVM assertions are enabled or not
-#assertions = false
+assertions = true

 # Indicates whether ccache is used when building LLVM
 #ccache = false
@@ -65,7 +65,7 @@
 #static-libstdcpp = false

 # Whether to use Ninja to build LLVM. This runs much faster than make.
-#ninja = true
+ninja = true

 # LLVM targets to build support for.
 # Note: this is NOT related to Rust compilation targets. However, as Rust is
@@ -117,7 +117,7 @@
 #use-libcxx = true

 # The value specified here will be passed as `-DLLVM_USE_LINKER` to CMake.
-#use-linker = "lld"
+use-linker = "lld"

 # Whether or not to specify `-DLLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN=YES`
 #allow-old-toolchain = false
@@ -147,7 +147,7 @@
 # Defaults to `host`. If you set this explicitly, you likely want to add all
 # host triples to this list as well in order for those host toolchains to be
 # able to compile programs for their native target.
-#target = ["x86_64-unknown-linux-gnu"]
+target = ["x86_64-pc-windows-gnu"]

 # Use this directory to store build artifacts.
 # You can use "$ROOT" to indicate the root of the git repository.
@@ -168,7 +168,7 @@
 # Flag to specify whether any documentation is built. If false, rustdoc and
 # friends will still be compiled but they will not be used to generate any
 # documentation.
-#docs = true
+docs = false

 # Indicate whether the compiler should be documented in addition to the standard
 # library and facade crates.
@@ -337,7 +337,7 @@
 # binary, otherwise they are omitted.
 #
 # Defaults to rust.debug value
-#debug-assertions = debug
+debug-assertions = true

 # Whether or not debug assertions are enabled for the standard library.
 # Overrides the `debug-assertions` option, if defined.
@@ -390,7 +390,7 @@
 #backtrace = true

 # Whether to always use incremental compilation when building rustc
-#incremental = false
+incremental = true

 # Build a multi-threaded rustc
 # FIXME(#75760): Some UI tests fail when this option is enabled.
@@ -455,7 +455,7 @@
 # LLD will not be used if we're cross linking.
 #
 # Explicitly setting the linker for a target will override this option when targeting MSVC.
-#use-lld = false
+use-lld = true

 # Indicates whether some LLVM tools, like llvm-objdump, will be made available in the
 # sysroot.
@@ -506,30 +506,38 @@
 # Each of the following options is scoped to the specific target triple in
 # question and is used for determining how to compile each target.
 # =============================================================================
+[target.x86_64-pc-windows-gnu]
+
+cc = "x86_64-w64-mingw32-gcc"
+cxx = "x86_64-w64-mingw32-g++"
+ar = "x86_64-w64-mingw32-ar"
+ranlib = "x86_64-w64-mingw32-ranlib"
+linker = "x86_64-w64-mingw32-gcc"
+
 [target.x86_64-unknown-linux-gnu]

 # C compiler to be used to compiler C code. Note that the
 # default value is platform specific, and if not specified it may also depend on
 # what platform is crossing to what platform.
-#cc = "cc"
+cc = "clang"

 # C++ compiler to be used to compiler C++ code (e.g. LLVM and our LLVM shims).
 # This is only used for host targets.
-#cxx = "c++"
+cxx = "clang++"

 # Archiver to be used to assemble static libraries compiled from C/C++ code.
 # Note: an absolute path should be used, otherwise LLVM build will break.
-#ar = "ar"
+ar = "llvm-ar"

 # Ranlib to be used to assemble static libraries compiled from C/C++ code.
 # Note: an absolute path should be used, otherwise LLVM build will break.
-#ranlib = "ranlib"
+ranlib = "llvm-ranlib"

 # Linker to be used to link Rust code. Note that the
 # default value is platform specific, and if not specified it may also depend on
 # what platform is crossing to what platform.
-# Setting this will override the `use-lld` option for Rust code when targeting MSVC.
-#linker = "cc"
+# Setting this will override the `use-lld` option for Rust code.
+linker = "clang"

 # Path to the `llvm-config` binary of the installation of a custom LLVM to link
 # against. Note that if this is specified we don't compile LLVM at all for this
