
% echo 'unsigned int max(unsigned int *ptr, unsigned int val) { return __atomic_fetch_max(ptr, val, __ATOMIC_RELAXED); }' | cheri-llvm clang -target mips64 -x c - -o - -S
	.text
	.abicalls
	.section	.mdebug.abi64,"",@progbits
	.nan	legacy
	.text
	.file	"-"
fatal error: error in backend: Cannot select: t22: i32,ch = AtomicLoadUMax<(load store monotonic 4 on %ir.0)> t21:1, t16, t21
  t16: i64,ch = load<(dereferenceable load 8 from %ir.ptr.addr)> t15, FrameIndex:i64<0>, undef:i64
    t10: i64 = FrameIndex<0>
    t12: i64 = undef
  t21: i32,ch = load<(dereferenceable load 4 from %ir..atomictmp)> t20, FrameIndex:i64<2>, undef:i64
    t18: i64 = FrameIndex<2>
    t12: i64 = undef
In function: max
