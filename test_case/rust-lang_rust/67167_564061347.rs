
% printf 'define i32 @max(i32 *%%ptr, i32 %%val) {\n %%1 = atomicrmw umax i32 *%%ptr, i32 %%val monotonic\n ret i32 %%1\n}' | llc -mtriple mips64
	.text
	.section	.mdebug.abi64,"",@progbits
	.nan	legacy
	.text
	.file	"<stdin>"
LLVM ERROR: Cannot select: t6: i32,ch = AtomicLoadUMax<(load store monotonic 4 on %ir.ptr)> t0, t2, t5
  t2: i64,ch = CopyFromReg t0, Register:i64 %0
    t1: i64 = Register %0
  t5: i32 = truncate t4
    t4: i64,ch = CopyFromReg t0, Register:i64 %1
      t3: i64 = Register %1
In function: max

% printf 'define i64 @max(i64 *%%ptr, i64 %%val) {\n %%1 = atomicrmw umax i64 *%%ptr, i64 %%val monotonic\n ret i64 %%1\n}' | llc -mtriple mips64
	.text
	.section	.mdebug.abi64,"",@progbits
	.nan	legacy
	.text
	.file	"<stdin>"
LLVM ERROR: Cannot select: t5: i64,ch = AtomicLoadUMax<(load store monotonic 8 on %ir.ptr)> t0, t2, t4
  t2: i64,ch = CopyFromReg t0, Register:i64 %0
    t1: i64 = Register %0
  t4: i64,ch = CopyFromReg t0, Register:i64 %1
    t3: i64 = Register %1
In function: max
