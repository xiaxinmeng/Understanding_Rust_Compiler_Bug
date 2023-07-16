
[santiago@archlinux tmp]$ valgrind --tool=massif rustc +stage1 test.rs 
==10838== Massif, a heap profiler
==10838== Copyright (C) 2003-2017, and GNU GPL'd, by Nicholas Nethercote
==10838== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
==10838== Command: rustc +stage1 -g test.rs
==10838== 
thread 'main' panicked at 'internal error: entered unreachable code: not all instructions were compiled! found uncompiled instruction: Compiled(Bytes(InstBytes { goto: 12, start: 101, end: 101 }))', target/cargo-home/registry/src/github.com-1ecc6299db9ec823/regex-0.2.6/src/compile.rs:794:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.
==10838== 
==10838== Process terminating with default action of signal 11 (SIGSEGV): dumping core
==10838==  Access not within mapped region at address 0x1000000823
==10838==    at 0x63573E: je_huge_dalloc (rtree.h:152)
==10838==    by 0x2D96C3: core::ptr::drop_in_place (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x2E409A: regex::compile::Compiler::fill (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x2E0B97: regex::compile::Compiler::c (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x2E2BEE: regex::compile::Compiler::c_alternate (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x2E0B3D: regex::compile::Compiler::c (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x2DEA2D: regex::compile::Compiler::compile (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x2E9ECB: regex::exec::ExecBuilder::build (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x30208C: regex::re_unicode::Regex::new (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x2837BE: rustup_dist::dist::PartialToolchainDesc::from_str (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x2537DC: rustup::config::Cfg::resolve_toolchain (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x2414B3: rustup::toolchain::Toolchain::from (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x252536: rustup::config::Cfg::create_command_for_toolchain (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x1B39F9: rustup_init::run_rustup (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x1B16AF: rustup_init::main (in /home/santiago/.cargo/bin/rustc)
==10838==    by 0x61078E: __rust_maybe_catch_panic (lib.rs:101)
==10838==    by 0x6082BB: std::rt::lang_start (panicking.rs:459)
==10838==    by 0x58A8F49: (below main) (in /usr/lib/libc-2.26.so)
==10838==  If you believe this happened as a result of a stack
==10838==  overflow in your program's main thread (unlikely but
==10838==  possible), you can try to increase the size of the
==10838==  main thread stack using the --main-stacksize= flag.
==10838==  The main thread stack size used in this run was 8388608.
==10838== 
Segmentation fault (core dumped)
