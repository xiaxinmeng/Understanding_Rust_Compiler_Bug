diff
% diff config.toml.example config.toml
--- config.toml.example 2020-07-01 12:34:56.594252303 +0000
+++ config.toml 2020-07-01 12:55:19.771875118 +0000
@@ -36,7 +36,7 @@
 #assertions = false
 
 # Indicates whether ccache is used when building LLVM
-#ccache = false
+ccache = true
 # or alternatively ...
 #ccache = "/path/to/ccache"
 
@@ -52,7 +52,7 @@
 # Tell the LLVM build system to use Ninja instead of the platform default for
 # the generated build system. This can sometimes be faster than make, for
 # example.
-#ninja = false
+ninja = true
 
 # LLVM targets to build support for.
 # Note: this is NOT related to Rust compilation targets. However, as Rust is
@@ -63,7 +63,7 @@
 # support. You'll need to write a target specification at least, and most
 # likely, teach rustc about the C ABI of the target. Get in touch with the
 # Rust team and file an issue if you need assistance in porting!
-#targets = "AArch64;ARM;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86"
+targets = "AArch64;ARM;Mips;X86"
 
 # LLVM experimental targets to build support for. These targets are specified in
 # the same format as above, but since these targets are experimental, they are
@@ -104,7 +104,7 @@
 #use-libcxx = true
 
 # The value specified here will be passed as `-DLLVM_USE_LINKER` to CMake.
-#use-linker = "lld"
+use-linker = "lld"
 
 # Whether or not to specify `-DLLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN=YES`
 #allow-old-toolchain = false
@@ -481,11 +481,11 @@
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
