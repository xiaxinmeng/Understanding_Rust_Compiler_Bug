
--- rustc-1.25.0-src/src/libcompiler_builtins/compiler-rt/lib/builtins/ctzdi2.c
+++ rustc-1.25.0-src/src/libcompiler_builtins/compiler-rt/lib/builtins/ctzdi2.c
@@ -16,7 +16,7 @@

 /* Returns: the number of trailing 0-bits  */

-#if !defined(__clang__) && (defined(__sparc64__) || defined(__mips64) || defined(__riscv__))
+#if !defined(__clang__) && (defined(__sparcv9) || defined(__sparc64__) || defined(__mips64) || defined(__riscv__))
 /* gcc resolves __builtin_ctz -> __ctzdi2 leading to infinite recursion */
 #define __builtin_ctz(a) __ctzsi2(a)
 extern si_int __ctzsi2(si_int);
 

