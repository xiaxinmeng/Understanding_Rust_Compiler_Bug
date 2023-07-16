diff
--- config.toml.example 2020-01-10 12:45:28.064159323 +0000
+++ config.toml 2020-01-10 18:40:23.019243486 +0000
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
+targets = "AArch64;ARM;X86;RISCV"
 
 # LLVM experimental targets to build support for. These targets are specified in
 # the same format as above, but since these targets are experimental, they are
@@ -335,7 +335,7 @@
 #backtrace = true
 
 # Whether to always use incremental compilation when building rustc
-#incremental = false
+incremental = true
 
 # Build a multi-threaded rustc
 #parallel-compiler = false
