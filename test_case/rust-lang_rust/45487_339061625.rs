
#if (defined(__sparc64__) || defined(__mips_n64) || defined(__mips_o64) || defined(__riscv__) \
+		|| (_MIPS_SIM == _ABI64) || (_MIPS_SIM == _ABIO64))
+si_int __clzsi2(si_int);
+si_int __ctzsi2(si_int);
+#define	__builtin_clz __clzsi2
+#define	__builtin_ctz __ctzsi2
+#endif /* sparc64 || mips_n64 || mips_o64 || riscv */
