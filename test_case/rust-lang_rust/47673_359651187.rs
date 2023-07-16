
$ valgrind rustc /tmp/test.rs                
==4720== Memcheck, a memory error detector
==4720== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==4720== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
==4720== Command: rustc /tmp/test.rs
==4720== 
==4720== Invalid read of size 8
==4720==    at 0x682DA2: rallocx (arena.h:809)
==4720==    by 0x67D240: __rde_realloc (lib.rs:170)
==4720==    by 0x295008: <alloc::raw_vec::RawVec<T, A>>::double (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29E683: regex::compile::Compiler::c_class (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29F2D7: regex::compile::Compiler::c_concat (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29FB5C: regex::compile::Compiler::c_repeat (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29CAEA: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29CEA6: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29F5D0: regex::compile::Compiler::c_alternate (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29D46D: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29B0DD: regex::compile::Compiler::compile (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2A6B81: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==  Address 0x6200020 is 32 bytes before a block of size 560 in arena "client"
==4720== 
==4720== Invalid read of size 8
==4720==    at 0x682CF7: rallocx (arena.h:809)
==4720==    by 0x67D240: __rde_realloc (lib.rs:170)
==4720==    by 0x295008: <alloc::raw_vec::RawVec<T, A>>::double (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29E683: regex::compile::Compiler::c_class (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29F2D7: regex::compile::Compiler::c_concat (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29FB5C: regex::compile::Compiler::c_repeat (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29CAEA: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29CEA6: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29F5D0: regex::compile::Compiler::c_alternate (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29D46D: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29B0DD: regex::compile::Compiler::compile (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2A6B81: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==  Address 0x6200020 is 32 bytes before a block of size 560 in arena "client"
==4720== 
==4720== Invalid read of size 8
==4720==    at 0x4C33FFE: memcpy@GLIBC_2.2.5 (vg_replace_strmem.c:1021)
==4720==    by 0x68C3FB: je_arena_ralloc (arena.c:3375)
==4720==    by 0x682D68: je_iralloct (jemalloc_internal.h:1259)
==4720==    by 0x682D68: rallocx (jemalloc.c:2414)
==4720==    by 0x67D240: __rde_realloc (lib.rs:170)
==4720==    by 0x295008: <alloc::raw_vec::RawVec<T, A>>::double (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29E683: regex::compile::Compiler::c_class (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29F2D7: regex::compile::Compiler::c_concat (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29FB5C: regex::compile::Compiler::c_repeat (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29CAEA: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29CEA6: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29F5D0: regex::compile::Compiler::c_alternate (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29D46D: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==  Address 0x6202b20 is 0 bytes after a block of size 256 alloc'd
==4720==    at 0x4C31A1E: calloc (vg_replace_malloc.c:711)
==4720==    by 0x67D295: __rde_alloc_zeroed (lib.rs:185)
==4720==    by 0x2BEDB6: regex::prog::Program::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29AC68: regex::compile::Compiler::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2A6885: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF1BA: regex::re_unicode::Regex::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2241B8: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F54DC: rustup::config::Cfg::resolve_toolchain (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1E22B3: rustup::toolchain::Toolchain::from (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F0D0E: rustup::config::Cfg::find_override (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F1DFF: rustup::config::Cfg::find_override_toolchain_or_default (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3BA0: rustup::config::Cfg::toolchain_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720== 
==4720== Invalid read of size 8
==4720==    at 0x4C33FF0: memcpy@GLIBC_2.2.5 (vg_replace_strmem.c:1021)
==4720==    by 0x68C3FB: je_arena_ralloc (arena.c:3375)
==4720==    by 0x682D68: je_iralloct (jemalloc_internal.h:1259)
==4720==    by 0x682D68: rallocx (jemalloc.c:2414)
==4720==    by 0x67D240: __rde_realloc (lib.rs:170)
==4720==    by 0x295008: <alloc::raw_vec::RawVec<T, A>>::double (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29E683: regex::compile::Compiler::c_class (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29F2D7: regex::compile::Compiler::c_concat (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29FB5C: regex::compile::Compiler::c_repeat (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29CAEA: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29CEA6: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29F5D0: regex::compile::Compiler::c_alternate (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29D46D: regex::compile::Compiler::c (in /home/dwrensha/.cargo/bin/rustc)
==4720==  Address 0x6202b28 is 8 bytes after a block of size 256 alloc'd
==4720==    at 0x4C31A1E: calloc (vg_replace_malloc.c:711)
==4720==    by 0x67D295: __rde_alloc_zeroed (lib.rs:185)
==4720==    by 0x2BEDB6: regex::prog::Program::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29AC68: regex::compile::Compiler::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2A6885: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF1BA: regex::re_unicode::Regex::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2241B8: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F54DC: rustup::config::Cfg::resolve_toolchain (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1E22B3: rustup::toolchain::Toolchain::from (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F0D0E: rustup::config::Cfg::find_override (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F1DFF: rustup::config::Cfg::find_override_toolchain_or_default (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3BA0: rustup::config::Cfg::toolchain_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720== 
==4720== Conditional jump or move depends on uninitialised value(s)
==4720==    at 0x2A7C4F: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF1BA: regex::re_unicode::Regex::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2241B8: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F54DC: rustup::config::Cfg::resolve_toolchain (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1E22B3: rustup::toolchain::Toolchain::from (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F0D0E: rustup::config::Cfg::find_override (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F1DFF: rustup::config::Cfg::find_override_toolchain_or_default (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3BA0: rustup::config::Cfg::toolchain_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3E29: rustup::config::Cfg::create_command_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1B5F5E: rustup_init::run_rustup (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1B3D1F: rustup_init::main (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x67D00C: __rust_maybe_catch_panic (lib.rs:99)
==4720== 
==4720== Invalid write of size 8
==4720==    at 0x4C33FF3: memcpy@GLIBC_2.2.5 (vg_replace_strmem.c:1021)
==4720==    by 0x295657: <thread_local::CachedThreadLocal<T>>::get_or_try_slow (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF54F: regex::re_unicode::Regex::locations (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF300: regex::re_unicode::Regex::captures (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x224264: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F54DC: rustup::config::Cfg::resolve_toolchain (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1E22B3: rustup::toolchain::Toolchain::from (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F0D0E: rustup::config::Cfg::find_override (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F1DFF: rustup::config::Cfg::find_override_toolchain_or_default (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3BA0: rustup::config::Cfg::toolchain_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3E29: rustup::config::Cfg::create_command_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1B5F5E: rustup_init::run_rustup (in /home/dwrensha/.cargo/bin/rustc)
==4720==  Address 0x6202da0 is 0 bytes after a block of size 256 alloc'd
==4720==    at 0x4C31A1E: calloc (vg_replace_malloc.c:711)
==4720==    by 0x67D295: __rde_alloc_zeroed (lib.rs:185)
==4720==    by 0x2BEDB6: regex::prog::Program::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29AC68: regex::compile::Compiler::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2A6A3C: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF1BA: regex::re_unicode::Regex::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2241B8: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F54DC: rustup::config::Cfg::resolve_toolchain (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1E22B3: rustup::toolchain::Toolchain::from (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F0D0E: rustup::config::Cfg::find_override (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F1DFF: rustup::config::Cfg::find_override_toolchain_or_default (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3BA0: rustup::config::Cfg::toolchain_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720== 
==4720== Invalid write of size 8
==4720==    at 0x4C33FF3: memcpy@GLIBC_2.2.5 (vg_replace_strmem.c:1021)
==4720==    by 0x68C3FB: je_arena_ralloc (arena.c:3375)
==4720==    by 0x682D68: je_iralloct (jemalloc_internal.h:1259)
==4720==    by 0x682D68: rallocx (jemalloc.c:2414)
==4720==    by 0x67D240: __rde_realloc (lib.rs:170)
==4720==    by 0x3095C2: <alloc::raw_vec::RawVec<T, A>>::double (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x314C65: regex_syntax::parser::Parser::parse_expr (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x32108D: regex_syntax::ExprBuilder::parse (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2A5D02: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF1BA: regex::re_unicode::Regex::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x223AE2: rustup_dist::dist::PartialTargetTriple::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x224376: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F54DC: rustup::config::Cfg::resolve_toolchain (in /home/dwrensha/.cargo/bin/rustc)
==4720==  Address 0x6202b20 is 0 bytes after a block of size 256 alloc'd
==4720==    at 0x4C31A1E: calloc (vg_replace_malloc.c:711)
==4720==    by 0x67D295: __rde_alloc_zeroed (lib.rs:185)
==4720==    by 0x2BEDB6: regex::prog::Program::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x29AC68: regex::compile::Compiler::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2A6885: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF1BA: regex::re_unicode::Regex::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2241B8: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F54DC: rustup::config::Cfg::resolve_toolchain (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1E22B3: rustup::toolchain::Toolchain::from (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F0D0E: rustup::config::Cfg::find_override (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F1DFF: rustup::config::Cfg::find_override_toolchain_or_default (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3BA0: rustup::config::Cfg::toolchain_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720== 
==4720== Invalid read of size 8
==4720==    at 0x682DA2: rallocx (arena.h:809)
==4720==    by 0x67D240: __rde_realloc (lib.rs:170)
==4720==    by 0x3095C2: <alloc::raw_vec::RawVec<T, A>>::double (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x314C65: regex_syntax::parser::Parser::parse_expr (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x32108D: regex_syntax::ExprBuilder::parse (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2A5D02: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF1BA: regex::re_unicode::Regex::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x223AE2: rustup_dist::dist::PartialTargetTriple::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x224376: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F54DC: rustup::config::Cfg::resolve_toolchain (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1E22B3: rustup::toolchain::Toolchain::from (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F0D0E: rustup::config::Cfg::find_override (in /home/dwrensha/.cargo/bin/rustc)
==4720==  Address 0x6200020 is 32 bytes before a block of size 560 in arena "client"
==4720== 

valgrind: m_mallocfree.c:307 (get_bszB_as_is): Assertion 'bszB_lo == bszB_hi' failed.
valgrind: Heap block lo/hi size mismatch: lo = 320, hi = 107229459.
This is probably caused by your program erroneously writing past the
end of a heap block and corrupting heap metadata.  If you fix any
invalid writes reported by Memcheck, this assertion failure will
probably go away.  Please try that before reporting this as a bug.


host stacktrace:
==4720==    at 0x580410A8: ??? (in /usr/lib64/valgrind/memcheck-amd64-linux)
==4720==    by 0x580411B4: ??? (in /usr/lib64/valgrind/memcheck-amd64-linux)
==4720==    by 0x58041330: ??? (in /usr/lib64/valgrind/memcheck-amd64-linux)
==4720==    by 0x5804F17D: ??? (in /usr/lib64/valgrind/memcheck-amd64-linux)
==4720==    by 0x5803A24B: ??? (in /usr/lib64/valgrind/memcheck-amd64-linux)
==4720==    by 0x58038A93: ??? (in /usr/lib64/valgrind/memcheck-amd64-linux)
==4720==    by 0x5803CD3A: ??? (in /usr/lib64/valgrind/memcheck-amd64-linux)
==4720==    by 0x58037E3B: ??? (in /usr/lib64/valgrind/memcheck-amd64-linux)
==4720==    by 0x1003569904: ???
==4720==    by 0x1002FB1F2F: ???
==4720==    by 0x100200834F: ???
==4720==    by 0x3203B3: regex_syntax::parser::Parser::lit (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x100200834F: ???

sched status:
  running_tid=1

Thread 1: status = VgTs_Runnable (lwpid 4720)
==4720==    at 0x314C8A: regex_syntax::parser::Parser::parse_expr (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x32108D: regex_syntax::ExprBuilder::parse (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2A5D02: regex::exec::ExecBuilder::build (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x2BF1BA: regex::re_unicode::Regex::new (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x223AE2: rustup_dist::dist::PartialTargetTriple::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x224376: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F54DC: rustup::config::Cfg::resolve_toolchain (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1E22B3: rustup::toolchain::Toolchain::from (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F0D0E: rustup::config::Cfg::find_override (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F1DFF: rustup::config::Cfg::find_override_toolchain_or_default (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3BA0: rustup::config::Cfg::toolchain_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1F3E29: rustup::config::Cfg::create_command_for_dir (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1B5F5E: rustup_init::run_rustup (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x1B3D1F: rustup_init::main (in /home/dwrensha/.cargo/bin/rustc)
==4720==    by 0x67D00C: __rust_maybe_catch_panic (lib.rs:99)
==4720==    by 0x67672B: try<(),closure> (panicking.rs:459)
==4720==    by 0x67672B: catch_unwind<closure,()> (panic.rs:361)
==4720==    by 0x67672B: std::rt::lang_start (rt.rs:59)
==4720==    by 0x58B6009: (below main) (in /usr/lib64/libc-2.26.so)


Note: see also the FAQ in the source distribution.
It contains workarounds to several common problems.
In particular, if Valgrind aborted or crashed after
identifying problems in your program, there's a good chance
that fixing those problems will prevent Valgrind aborting or
crashing, especially if it happened in m_mallocfree.c.

If that doesn't help, please report this bug to: www.valgrind.org

In the bug report, send all the above text, the valgrind
version, and what OS and version you are using.  Thanks.
