
--- config.toml.example	2019-07-04 15:20:12.472184560 +0200
+++ config.toml	2019-07-24 18:34:31.037396329 +0200
@@ -291,11 +291,11 @@
 # Number of codegen units to use for each compiler invocation. A value of 0
 # means "the number of cores on this machine", and 1+ is passed through to the
 # compiler.
-#codegen-units = 1
+codegen-units = 0
 
 # Sets the number of codegen units to build the standard library with,
 # regardless of what the codegen-unit setting for the rest of the compiler is.
-#codegen-units-std = 1
+codegen-units-std = 0
 
 # Whether or not debug assertions are enabled for the compiler and standard
 # library.
@@ -327,7 +327,7 @@
 #backtrace = true
 
 # Whether to always use incremental compilation when building rustc
-#incremental = false
+incremental = true
 
 # Build a multi-threaded rustc
 #parallel-compiler = false
