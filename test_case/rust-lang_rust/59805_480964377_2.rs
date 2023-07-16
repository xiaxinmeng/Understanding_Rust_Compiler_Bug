
(gdb) c
Continuing.

Program received signal SIGILL, Illegal instruction.
0x2a00bd94 in std::panicking::rust_panic_with_hook::h40a77253872948e8 () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/fmt/mod.rs:307
307	/rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/fmt/mod.rs: No such file or directory.
(gdb) bt
#0  0x2a00bd94 in std::panicking::rust_panic_with_hook::h40a77253872948e8 () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/fmt/mod.rs:307
#1  0x2a00b648 in std::panicking::continue_panic_fmt::hec94fc8e5daf641b () at src/libstd/panicking.rs:385
#2  0x2a018f28 in rust_begin_unwind () at src/libstd/panicking.rs:312
#3  0x2a01d85c in core::panicking::panic_fmt::h74ee8034b317ceed () at src/libcore/panicking.rs:85
#4  0x2a00b270 in core::result::unwrap_failed::he29400c620ac986f () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/macros.rs:16
#5  0x2a00a9ec in expect<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,core::cell::BorrowMutError> () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/result.rs:825
#6  borrow_mut<core::option::Option<std::sys_common::thread_info::ThreadInfo>> () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/cell.rs:876
#7  {{closure}}<std::thread::Thread,closure> () at src/libstd/sys_common/thread_info.rs:18
#8  _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::try_with::h24a826ffafbf3074 () at src/libstd/thread/local.rs:299
#9  0x2a00b9a0 in with<std::thread::Thread,closure> () at src/libstd/sys_common/thread_info.rs:16
#10 current_thread () at src/libstd/sys_common/thread_info.rs:29
#11 default_hook () at src/libstd/panicking.rs:186
#12 std::panicking::rust_panic_with_hook::h40a77253872948e8 () at src/libstd/panicking.rs:478
#13 0x2a00b648 in std::panicking::continue_panic_fmt::hec94fc8e5daf641b () at src/libstd/panicking.rs:385
#14 0x2a018f28 in rust_begin_unwind () at src/libstd/panicking.rs:312
#15 0x2a01d85c in core::panicking::panic_fmt::h74ee8034b317ceed () at src/libcore/panicking.rs:85
#16 0x2a00b270 in core::result::unwrap_failed::he29400c620ac986f () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/macros.rs:16
#17 0x2a00a9cc in expect<core::cell::RefMut<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,core::cell::BorrowMutError> () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/result.rs:825
#18 borrow_mut<core::option::Option<std::sys_common::thread_info::ThreadInfo>> () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/cell.rs:876
#19 {{closure}}<std::thread::Thread,closure> () at src/libstd/sys_common/thread_info.rs:23
#20 _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::try_with::h24a826ffafbf3074 () at src/libstd/thread/local.rs:299
#21 0x2a00b9a0 in with<std::thread::Thread,closure> () at src/libstd/sys_common/thread_info.rs:16
#22 current_thread () at src/libstd/sys_common/thread_info.rs:29
#23 default_hook () at src/libstd/panicking.rs:186
#24 std::panicking::rust_panic_with_hook::h40a77253872948e8 () at src/libstd/panicking.rs:478
#25 0x2a00b648 in std::panicking::continue_panic_fmt::hec94fc8e5daf641b () at src/libstd/panicking.rs:385
#26 0x2a018f28 in rust_begin_unwind () at src/libstd/panicking.rs:312
#27 0x2a01d85c in core::panicking::panic_fmt::h74ee8034b317ceed () at src/libcore/panicking.rs:85
#28 0x2a00ad28 in core::result::unwrap_failed::hfdeec756f5ccb9c2 () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/macros.rs:16
#29 0x2a018bb0 in expect<core::cell::Ref<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,core::cell::BorrowError> () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/result.rs:825
#30 borrow<core::option::Option<std::sys_common::thread_info::ThreadInfo>> () at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libcore/cell.rs:795
#31 {{closure}} () at src/libstd/sys_common/thread_info.rs:37
#32 try_with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,()> () at src/libstd/thread/local.rs:299
#33 with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,()> () at src/libstd/thread/local.rs:245
#34 std::sys_common::thread_info::set::hd1facf06aa7d5486 () at src/libstd/sys_common/thread_info.rs:37
#35 0x2a01917c in std::rt::lang_start_internal::h747c58e2c78f4f4f () at src/libstd/rt.rs:41
#36 0x2a023194 in main ()
(gdb) 

(gdb) disassemble
Dump of assembler code for function std::panicking::rust_panic_with_hook::h40a77253872948e8:
---Type <return> to continue, or q <return> to quit---
   0x2a00bd28 <+1724>:	cmp	r0, #0
   0x2a00bd2c <+1728>:	movne	r0, r4
   0x2a00bd30 <+1732>:	blne	0x2a021c68 <__rust_dealloc>
   0x2a00bd34 <+1736>:	ldrex	r0, [r5]
   0x2a00bd38 <+1740>:	sub	r0, r0, #1
   0x2a00bd3c <+1744>:	strex	r1, r0, [r5]
   0x2a00bd40 <+1748>:	cmp	r1, #0
   0x2a00bd44 <+1752>:	bne	0x2a00bd34 <std::panicking::rust_panic_with_hook::h40a77253872948e8+1736>
   0x2a00bd48 <+1756>:	ldr	r0, [pc, #584]	; 0x2a00bf98 <std::panicking::rust_panic_with_hook::h40a77253872948e8+2348>
   0x2a00bd4c <+1760>:	add	r0, pc, r0
   0x2a00bd50 <+1764>:	bl	0x2a00257c <pthread_rwlock_unlock@plt>
   0x2a00bd54 <+1768>:	cmp	r11, #2
   0x2a00bd58 <+1772>:	bcc	0x2a00bd9c <std::panicking::rust_panic_with_hook::h40a77253872948e8+1840>
   0x2a00bd5c <+1776>:	mov	r0, #0
   0x2a00bd60 <+1780>:	str	r0, [sp, #24]
   0x2a00bd64 <+1784>:	ldr	r1, [pc, #560]	; 0x2a00bf9c <std::panicking::rust_panic_with_hook::h40a77253872948e8+2352>
   0x2a00bd68 <+1788>:	add	r1, pc, r1
   0x2a00bd6c <+1792>:	str	r1, [sp, #20]
   0x2a00bd70 <+1796>:	str	r0, [sp, #16]
   0x2a00bd74 <+1800>:	str	r0, [sp, #12]
   0x2a00bd78 <+1804>:	mov	r0, #1
   0x2a00bd7c <+1808>:	str	r0, [sp, #8]
   0x2a00bd80 <+1812>:	ldr	r0, [pc, #536]	; 0x2a00bfa0 <std::panicking::rust_panic_with_hook::h40a77253872948e8+2356>
   0x2a00bd84 <+1816>:	add	r0, pc, r0
   0x2a00bd88 <+1820>:	str	r0, [sp, #4]
   0x2a00bd8c <+1824>:	add	r0, sp, #4
   0x2a00bd90 <+1828>:	bl	0x2a00bfc8 <std::sys_common::util::dumb_print::hc61004dc31357b07>
=> 0x2a00bd94 <+1832>:	udf	#65006	; 0xfdee
   0x2a00bd98 <+1836>:	udf	#65006	; 0xfdee
   0x2a00bd9c <+1840>:	mov	r0, r9
   0x2a00bda0 <+1844>:	mov	r1, r8
   0x2a00bda4 <+1848>:	bl	0x2a00c004 <rust_panic>
   0x2a00bda8 <+1852>:	udf	#65006	; 0xfdee
   0x2a00bdac <+1856>:	ldr	r0, [pc, #404]	; 0x2a00bf48 <std::panicking::rust_panic_with_hook::h40a77253872948e8+2268>
   0x2a00bdb0 <+1860>:	add	r1, sp, #116	; 0x74
   0x2a00bdb4 <+1864>:	add	r0, pc, r0
   0x2a00bdb8 <+1868>:	str	r0, [sp, #68]	; 0x44
   0x2a00bdbc <+1872>:	str	r1, [sp, #64]	; 0x40
   0x2a00bdc0 <+1876>:	add	r1, sp, #56	; 0x38
   0x2a00bdc4 <+1880>:	str	r0, [sp, #60]	; 0x3c
   0x2a00bdc8 <+1884>:	add	r0, sp, #108	; 0x6c
   0x2a00bdcc <+1888>:	str	r0, [sp, #56]	; 0x38
   0x2a00bdd0 <+1892>:	mov	r0, #2
   0x2a00bdd4 <+1896>:	str	r0, [sp, #52]	; 0x34
   0x2a00bdd8 <+1900>:	str	r1, [sp, #48]	; 0x30
   0x2a00bddc <+1904>:	str	r0, [sp, #44]	; 0x2c
   0x2a00bde0 <+1908>:	ldr	r0, [pc, #356]	; 0x2a00bf4c <std::panicking::rust_panic_with_hook::h40a77253872948e8+2272>
   0x2a00bde4 <+1912>:	add	r0, pc, r0
   0x2a00bde8 <+1916>:	str	r0, [sp, #40]	; 0x28
   0x2a00bdec <+1920>:	mov	r0, #3
   0x2a00bdf0 <+1924>:	str	r0, [sp, #36]	; 0x24
   0x2a00bdf4 <+1928>:	ldr	r0, [pc, #340]	; 0x2a00bf50 <std::panicking::rust_panic_with_hook::h40a77253872948e8+2276>
   0x2a00bdf8 <+1932>:	add	r0, pc, r0
   0x2a00bdfc <+1936>:	str	r0, [sp, #32]
   0x2a00be00 <+1940>:	ldr	r1, [pc, #332]	; 0x2a00bf54 <std::panicking::rust_panic_with_hook::h40a77253872948e8+2280>
   0x2a00be04 <+1944>:	add	r0, sp, #32
   0x2a00be08 <+1948>:	add	r1, pc, r1
   0x2a00be0c <+1952>:	bl	0x2a00b590 <std::panicking::begin_panic_fmt::h6c3852466bd3939b>
   0x2a00be10 <+1956>:	udf	#65006	; 0xfdee
   0x2a00be14 <+1960>:	bl	0x2a00c374 <core::result::unwrap_failed::hf4efc62e13016b4a>
   0x2a00be18 <+1964>:	b	0x2a00be58 <std::panicking::rust_panic_with_hook::h40a77253872948e8+2028>
   0x2a00be1c <+1968>:	ldr	r0, [pc, #308]	; 0x2a00bf58 <std::panicking::rust_panic_with_hook::h40a77253872948e8+2284>
   0x2a00be20 <+1972>:	add	r0, pc, r0
   0x2a00be24 <+1976>:	bl	0x2a00257c <pthread_rwlock_unlock@plt>
   0x2a00be28 <+1980>:	ldr	r0, [pc, #300]	; 0x2a00bf5c <std::panicking::rust_panic_with_hook::h40a77253872948e8+2288>
   0x2a00be2c <+1984>:	mov	r1, #41	; 0x29
   0x2a00be30 <+1988>:	ldr	r2, [pc, #296]	; 0x2a00bf60 <std::panicking::rust_panic_with_hook::h40a77253872948e8+2292>
   0x2a00be34 <+1992>:	add	r0, pc, r0
   0x2a00be38 <+1996>:	add	r2, pc, r2
   0x2a00be3c <+2000>:	bl	0x2a00b4cc <std::panicking::begin_panic::h6c11457bbc159610>
   0x2a00be40 <+2004>:	udf	#65006	; 0xfdee
   0x2a00be44 <+2008>:	bl	0x2a00b1ec <core::result::unwrap_failed::he29400c620ac986f>
