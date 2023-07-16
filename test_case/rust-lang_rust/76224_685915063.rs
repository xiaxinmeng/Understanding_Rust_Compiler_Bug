patch
--- i/compiler-rt/lib/profile/GCDAProfiling.c
+++ w/compiler-rt/lib/profile/GCDAProfiling.c
@@ -628,6 +628,9 @@ void llvm_writeout_files(void) {
   }
 }
 
+#if defined(__GNUC__) && __GNUC__ >= 9
+#pragma GCC diagnostic ignored "-Wprio-ctor-dtor"
+#endif
 #ifndef _WIN32
 // __attribute__((destructor)) and destructors whose priorities are greater than
 // 100 run before this function and can thus be tracked. The priority is
