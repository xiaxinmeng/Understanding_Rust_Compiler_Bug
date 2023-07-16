
strace: Process 24927 attached
[pid 24927] execve("/scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/bin/rustc", ["/scratch/user/bsferraz/sys/insta"..., "--crate-name", "core", "--edition=2021", "library/core/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,"..., "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3", "-C", "embed-bitcode=no", "-C", "debuginfo=0", "-Zunstable-options", "--check-cfg", "names()", "--check-cfg", "values()", "-C", "metadata=6fb61b0a462bb07a", "-C", "extra-filename=-6fb61b0a462bb07a", "--out-dir", "/scratch/user/bsferraz/sys/insta"..., "--target", "x86_64-unknown-linux-gnu", "-C", "incremental=/scratch/user/bsferr"..., "-L", ...], 0x2ac785fa9c20 /* 111 vars */) = 0
 > /scratch/user/bsferraz/sys/lib/libc.so.6(execve+0x7) [0xd4677]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__execvpe_common+0x55) [0xd4d75]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__spawni_child+0x1fc) [0xf6a9c]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__clone+0x40) [0x108390]
[pid 24927] getrandom(0x2b4b01b80478, 8, GRND_NONBLOCK) = -1 ENOSYS (Function not implemented)
 > /scratch/user/bsferraz/sys/lib/libc.so.6(getrandom+0x10) [0x3ef50]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(ptmalloc_init.part.0+0x36) [0x94fb6]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(malloc+0x215) [0x987c5]
 > /scratch/user/bsferraz/sys/lib/libstdc++.so.6.0.30(_GLOBAL__sub_I_eh_alloc.cc+0x3a) [0xad83a]
 > /scratch/user/bsferraz/sys/lib/ld-linux-x86-64.so.2(call_init+0xbe) [0x4b6e]
 > /scratch/user/bsferraz/sys/lib/ld-linux-x86-64.so.2(_dl_init+0x74) [0x4c54]
 > /scratch/user/bsferraz/sys/lib/ld-linux-x86-64.so.2(_dl_help+0x540) [0x1a740]
[pid 24927] getrandom(0x7ffe678766f0, 16, GRND_INSECURE) = -1 ENOSYS (Function not implemented)
 > /scratch/user/bsferraz/sys/lib/libc.so.6(getrandom+0x10) [0x3ef50]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN3std3sys4unix4rand3imp10fill_bytes17hf824096236e3c8cdE+0xfe) [0xbfe6e]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN3std3sys4unix4rand19hashmap_random_keys17h7d2882b5a043e5a7E+0x18) [0x77d48]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMNtNtNtCs6Z0hJvZQRbe_3std6thread5local4lazyINtB3_12LazyKeyInnerINtNtCs9ZTcLekmlwh_4core4cell4CellTyyEEE10initializeNCNvNvNvMs1h_NtNtNtB9_11collections4hash3mapNtB26_11RandomState3new4KEYS7___getit0ECsiyanh6KVFCt_18tracing_subscriber+0x27) [0x15d76c7]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMs2_NtNtCs6Z0hJvZQRbe_3std6thread5localINtB6_8LocalKeyINtNtCs9ZTcLekmlwh_4core4cell4CellTyyEEE4withNCNvMs1h_NtNtNtBa_11collections4hash3mapNtB1M_11RandomState3new0B2h_ECsiyanh6KVFCt_18tracing_subscriber+0x8) [0x15cf158]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvXs2_NtNtCsiyanh6KVFCt_18tracing_subscriber6filter3envNtB5_9EnvFilterNtNtCs9ZTcLekmlwh_4core7default7Default7default+0xacb) [0x15e09fb]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvCskiQyWqZNAkg_9rustc_log15init_env_logger+0x3f) [0x775cff]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvCskiQyWqZNAkg_9rustc_log21init_rustc_env_logger+0x16) [0x775cb6]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvCsgtDLS7mlP0r_12rustc_driver21init_rustc_env_logger+0x10) [0x705c50]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvCsgtDLS7mlP0r_12rustc_driver4main+0x2e) [0x705d8e]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(_RNvCslkWIyLERyqt_10rustc_main4main+0xd) [0x117d]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(_RINvNtNtCs6Z0hJvZQRbe_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECslkWIyLERyqt_10rustc_main+0x3) [0x1133]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(_RNCINvNtCs6Z0hJvZQRbe_3std2rt10lang_startuE0CslkWIyLERyqt_10rustc_main.llvm.17190305638522734720+0x9) [0x1159]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN3std9panicking3try17h4291ad4541d9d623E+0xe) [0x7e67e]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN3std2rt19lang_start_internal17hfe16f69e546ff90aE+0x2b) [0x70eab]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(main+0x22) [0x11a2]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__libc_start_call_main+0x7a) [0x271ca]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__libc_start_main+0x85) [0x27285]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/bin/rustc(_start+0x21) [0x1061]
strace: Process 24928 attached
[pid 24928] getrandom(NULL, 0, GRND_NONBLOCK) = -1 ENOSYS (Function not implemented)
 > /scratch/user/bsferraz/sys/lib/libc.so.6(syscall+0x19) [0x101169]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvNtCs6ipgwa2hmtb_9getrandom3imp22is_getrandom_available+0x17) [0x29179a7]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMs_NtCs6ipgwa2hmtb_9getrandom4utilNtB5_8LazyBool11unsync_initNvNtB7_3imp22is_getrandom_availableEB7_+0x1a) [0x291790a]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvCs6ipgwa2hmtb_9getrandom9getrandom+0x1b) [0x291718b]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvXs_NtCsk7JkiS2p4n3_9rand_core2osNtB4_5OsRngNtB6_7RngCore14try_fill_bytes+0xd) [0x2900f8d]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvYNtNtCsf8kmfGgRRFk_11rand_chacha6chacha12ChaCha12CoreNtCsk7JkiS2p4n3_9rand_core11SeedableRng8from_rngNtNtBV_2os5OsRngECsh7vZgft2gn4_4rand+0x2d) [0x28fb93d]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMNtNtNtCs6Z0hJvZQRbe_3std6thread5local4lazyINtB3_12LazyKeyInnerINtNtCsjznm9VTDAtw_5alloc2rc2RcINtNtCs9ZTcLekmlwh_4core4cell10UnsafeCellINtNtNtNtCsh7vZgft2gn4_4rand4rngs7adapter9reseeding12ReseedingRngNtNtCsf8kmfGgRRFk_11rand_chacha6chacha12ChaCha12CoreNtNtCsk7JkiS2p4n3_9rand_core2os5OsRngEEEE10initializeNCNvNvNtB2k_6thread14THREAD_RNG_KEY7___getit0EB2m_+0x2d) [0x28fb5ad]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMs2_NtNtCs6Z0hJvZQRbe_3std6thread5localINtB6_8LocalKeyINtNtCsjznm9VTDAtw_5alloc2rc2RcINtNtCs9ZTcLekmlwh_4core4cell10UnsafeCellINtNtNtNtCsh7vZgft2gn4_4rand4rngs7adapter9reseeding12ReseedingRngNtNtCsf8kmfGgRRFk_11rand_chacha6chacha12ChaCha12CoreNtNtCsk7JkiS2p4n3_9rand_core2os5OsRngEEEE4withNCNvNtB2b_6thread10thread_rng0BU_EB2d_+0x8) [0x28fb6e8]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvNtNtCs27XZnfkF81m_17rustc_incremental7persist2fs25prepare_session_directory+0x36f) [0x1ccbc7f]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtCskAKw8pI6xjz_15rustc_interface6passes16register_pluginsRDG0_INtNtNtCs9ZTcLekmlwh_4core3ops8function2FnTRL1_NtNtCsis02UuLC9ZB_13rustc_session7session7SessionQL0_NtNtCs5ZViw5UNe73_10rustc_lint7context9LintStoreEEp6OutputuNtNtB1a_6marker4SyncNtB3E_4SendEL_EB4_+0x278) [0x7d6a68]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMNtCskAKw8pI6xjz_15rustc_interface7queriesINtB3_5QueryTNtNtCscPPShaunMgW_9rustc_ast3ast5CrateINtNtCsjznm9VTDAtw_5alloc2rc2RcNtNtCs5ZViw5UNe73_10rustc_lint7context9LintStoreEEE7computeNCNvMs0_B3_NtB3_7Queries16register_plugins0EB5_+0x1ef) [0x7f1cef]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMs2_NtCskAKw8pI6xjz_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0s0_0INtNtCs9ZTcLekmlwh_4core6result6ResultINtNtB2f_6option6OptionNtB6_6LinkerENtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedEEB1n_+0x26c) [0x6ec06c]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvCslzmqIBn3a9T_10rustc_span15with_source_mapINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCINvNtCskAKw8pI6xjz_15rustc_interface9interface23create_compiler_and_runBJ_NCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0Es_0EB3q_+0x177) [0x716907]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtCskAKw8pI6xjz_15rustc_interface9interface23create_compiler_and_runINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0EB2B_+0x3d3) [0x6ecb33]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMs_Csath8bPdUG0i_10scoped_tlsINtB5_9ScopedKeyNtCslzmqIBn3a9T_10rustc_span14SessionGlobalsE3setNCINvNtCskAKw8pI6xjz_15rustc_interface9interface12run_compilerINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0B2y_EB41_+0x62) [0x6e94d2]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtNtCs6Z0hJvZQRbe_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvNtCskAKw8pI6xjz_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB1m_9interface12run_compilerINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0B32_E0B32_EB4v_+0x79) [0x71bb79]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtCs6Z0hJvZQRbe_3std9panicking3tryINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedEINtNtNtBF_5panic11unwind_safe16AssertUnwindSafeNCNCINvMNtB4_6threadNtB2S_7Builder16spawn_unchecked_NCINvNtCskAKw8pI6xjz_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB3H_9interface12run_compilerBA_NCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0BA_E0BA_Es_00EEB5u_+0x2e) [0x69eede]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNSNvYNCINvMNtCs6Z0hJvZQRbe_3std6threadNtBa_7Builder16spawn_unchecked_NCINvNtCskAKw8pI6xjz_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB1d_9interface12run_compilerINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0B2T_E0B2T_Es_0INtNtNtB2Y_3ops8function6FnOnceuE9call_once6vtableB4m_+0x90) [0x6e9030]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN90_$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$9call_once17h75833ee271cfd292E+0x18) [0xad018]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN3std3sys4unix6thread6Thread3new12thread_start17hceab38778623fe14E+0x17) [0xb23c7]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(start_thread+0x2c4) [0x88e84]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__clone+0x40) [0x108390]
strace: Process 24937 attached
[pid 24937] +++ exited with 0 +++
strace: Process 24966 attached
strace: Process 24967 attached
strace: Process 24978 attached
strace: Process 24979 attached
[pid 24978] +++ exited with 0 +++
strace: Process 24980 attached
[pid 24979] +++ exited with 0 +++
[pid 24980] +++ exited with 0 +++
strace: Process 24981 attached
[pid 24981] +++ exited with 0 +++
strace: Process 24982 attached
strace: Process 24983 attached
[pid 24982] +++ exited with 0 +++
[pid 24983] +++ exited with 0 +++
strace: Process 24984 attached
strace: Process 24985 attached
[pid 24984] +++ exited with 0 +++
[pid 24985] +++ exited with 0 +++
strace: Process 24986 attached
strace: Process 24987 attached
[pid 24986] +++ exited with 0 +++
strace: Process 24988 attached
[pid 24987] +++ exited with 0 +++
strace: Process 24989 attached
[pid 24988] +++ exited with 0 +++
[pid 24989] +++ exited with 0 +++
strace: Process 24990 attached
strace: Process 24991 attached
[pid 24990] +++ exited with 0 +++
strace: Process 24992 attached
[pid 24991] +++ exited with 0 +++
strace: Process 24993 attached
[pid 24992] +++ exited with 0 +++
strace: Process 24994 attached
[pid 24993] +++ exited with 0 +++
strace: Process 24995 attached
[pid 24994] +++ exited with 0 +++
strace: Process 24996 attached
[pid 24995] +++ exited with 0 +++
strace: Process 24997 attached
[pid 24996] +++ exited with 0 +++
[pid 24997] +++ exited with 0 +++
strace: Process 24998 attached
strace: Process 24999 attached
[pid 24998] +++ exited with 0 +++
strace: Process 25000 attached
[pid 24999] +++ exited with 0 +++
strace: Process 25001 attached
[pid 25000] +++ exited with 0 +++
strace: Process 25002 attached
[pid 25001] +++ exited with 0 +++
strace: Process 25003 attached
[pid 25002] +++ exited with 0 +++
[pid 25003] +++ exited with 0 +++
strace: Process 25004 attached
strace: Process 25005 attached
[pid 25004] +++ exited with 0 +++
strace: Process 25006 attached
[pid 25005] +++ exited with 0 +++
strace: Process 25007 attached
[pid 25006] +++ exited with 0 +++
strace: Process 25008 attached
[pid 25007] +++ exited with 0 +++
strace: Process 25009 attached
[pid 25008] +++ exited with 0 +++
[pid 25009] +++ exited with 0 +++
strace: Process 25010 attached
strace: Process 25011 attached
[pid 25010] +++ exited with 0 +++
strace: Process 25012 attached
[pid 25011] +++ exited with 0 +++
[pid 25012] +++ exited with 0 +++
strace: Process 25013 attached
strace: Process 25014 attached
[pid 25013] +++ exited with 0 +++
[pid 25014] +++ exited with 0 +++
strace: Process 25015 attached
strace: Process 25016 attached
[pid 25015] +++ exited with 0 +++
strace: Process 25018 attached
[pid 25016] +++ exited with 0 +++
strace: Process 25019 attached
[pid 25018] +++ exited with 0 +++
strace: Process 25020 attached
[pid 25019] +++ exited with 0 +++
strace: Process 25021 attached
[pid 25020] +++ exited with 0 +++
strace: Process 25022 attached
[pid 25021] +++ exited with 0 +++
strace: Process 25023 attached
[pid 25022] +++ exited with 0 +++
[pid 25023] +++ exited with 0 +++
strace: Process 25024 attached
strace: Process 25025 attached
[pid 25024] +++ exited with 0 +++
strace: Process 25026 attached
[pid 25025] +++ exited with 0 +++
strace: Process 25027 attached
[pid 25026] +++ exited with 0 +++
strace: Process 25028 attached
[pid 25027] +++ exited with 0 +++
strace: Process 25029 attached
[pid 25028] +++ exited with 0 +++
strace: Process 25030 attached
[pid 25029] +++ exited with 0 +++
strace: Process 25032 attached
[pid 25030] +++ exited with 0 +++
strace: Process 25033 attached
[pid 25032] +++ exited with 0 +++
strace: Process 25034 attached
[pid 25033] +++ exited with 0 +++
strace: Process 25035 attached
[pid 25034] +++ exited with 0 +++
strace: Process 25036 attached
[pid 25035] +++ exited with 0 +++
strace: Process 25037 attached
[pid 25036] +++ exited with 0 +++
strace: Process 25038 attached
[pid 25037] +++ exited with 0 +++
[pid 25038] +++ exited with 0 +++
strace: Process 25039 attached
strace: Process 25040 attached
[pid 25039] +++ exited with 0 +++
strace: Process 25041 attached
[pid 25040] +++ exited with 0 +++
strace: Process 25042 attached
[pid 25041] +++ exited with 0 +++
[pid 25042] +++ exited with 0 +++
strace: Process 25043 attached
strace: Process 25044 attached
[pid 25043] +++ exited with 0 +++
[pid 25044] +++ exited with 0 +++
strace: Process 25045 attached
strace: Process 25046 attached
[pid 25045] +++ exited with 0 +++
strace: Process 25047 attached
[pid 25046] +++ exited with 0 +++
strace: Process 25048 attached
[pid 25047] +++ exited with 0 +++
strace: Process 25049 attached
[pid 25048] +++ exited with 0 +++
[pid 25049] +++ exited with 0 +++
strace: Process 25050 attached
strace: Process 25051 attached
[pid 25050] +++ exited with 0 +++
[pid 25051] +++ exited with 0 +++
strace: Process 25052 attached
strace: Process 25053 attached
[pid 25052] +++ exited with 0 +++
strace: Process 25054 attached
[pid 25053] +++ exited with 0 +++
strace: Process 25055 attached
[pid 25054] +++ exited with 0 +++
strace: Process 25056 attached
[pid 25055] +++ exited with 0 +++
[pid 25056] +++ exited with 0 +++
strace: Process 25057 attached
strace: Process 25058 attached
[pid 25057] +++ exited with 0 +++
strace: Process 25059 attached
[pid 25058] +++ exited with 0 +++
strace: Process 25060 attached
[pid 25059] +++ exited with 0 +++
strace: Process 25061 attached
[pid 25060] +++ exited with 0 +++
strace: Process 25062 attached
[pid 25061] +++ exited with 0 +++
[pid 25062] +++ exited with 0 +++
strace: Process 25063 attached
strace: Process 25064 attached
[pid 25063] +++ exited with 0 +++
[pid 25064] +++ exited with 0 +++
strace: Process 25065 attached
strace: Process 25066 attached
[pid 25065] +++ exited with 0 +++
strace: Process 25067 attached
[pid 25066] +++ exited with 0 +++
strace: Process 25068 attached
[pid 25067] +++ exited with 0 +++
strace: Process 25069 attached
[pid 25068] +++ exited with 0 +++
strace: Process 25070 attached
[pid 25069] +++ exited with 0 +++
[pid 25070] +++ exited with 0 +++
strace: Process 25071 attached
strace: Process 25072 attached
[pid 25071] +++ exited with 0 +++
strace: Process 25073 attached
[pid 25072] +++ exited with 0 +++
strace: Process 25074 attached
[pid 25073] +++ exited with 0 +++
strace: Process 25075 attached
[pid 25074] +++ exited with 0 +++
strace: Process 25076 attached
[pid 25075] +++ exited with 0 +++
strace: Process 25077 attached
[pid 25076] +++ exited with 0 +++
[pid 25077] +++ exited with 0 +++
strace: Process 25078 attached
strace: Process 25080 attached
[pid 25078] +++ exited with 0 +++
strace: Process 25081 attached
[pid 25080] +++ exited with 0 +++
strace: Process 25082 attached
[pid 25081] +++ exited with 0 +++
strace: Process 25083 attached
[pid 25082] +++ exited with 0 +++
strace: Process 25084 attached
[pid 25083] +++ exited with 0 +++
strace: Process 25085 attached
[pid 25084] +++ exited with 0 +++
strace: Process 25086 attached
[pid 25085] +++ exited with 0 +++
strace: Process 25087 attached
[pid 25086] +++ exited with 0 +++
strace: Process 25088 attached
[pid 25087] +++ exited with 0 +++
strace: Process 25089 attached
[pid 25088] +++ exited with 0 +++
strace: Process 25090 attached
[pid 25089] +++ exited with 0 +++
strace: Process 25091 attached
[pid 25090] +++ exited with 0 +++
strace: Process 25092 attached
[pid 25091] +++ exited with 0 +++
strace: Process 25093 attached
[pid 25092] +++ exited with 0 +++
strace: Process 25094 attached
[pid 25093] +++ exited with 0 +++
strace: Process 25095 attached
[pid 25094] +++ exited with 0 +++
strace: Process 25096 attached
[pid 25095] +++ exited with 0 +++
strace: Process 25097 attached
[pid 25096] +++ exited with 0 +++
strace: Process 25098 attached
[pid 25097] +++ exited with 0 +++
strace: Process 25099 attached
[pid 25098] +++ exited with 0 +++
strace: Process 25100 attached
[pid 25099] +++ exited with 0 +++
strace: Process 25101 attached
[pid 25100] +++ exited with 0 +++
strace: Process 25102 attached
[pid 25101] +++ exited with 0 +++
strace: Process 25103 attached
[pid 25102] +++ exited with 0 +++
strace: Process 25104 attached
[pid 25103] +++ exited with 0 +++
strace: Process 25105 attached
[pid 25104] +++ exited with 0 +++
strace: Process 25106 attached
[pid 25105] +++ exited with 0 +++
strace: Process 25107 attached
[pid 25106] +++ exited with 0 +++
strace: Process 25108 attached
[pid 25107] +++ exited with 0 +++
strace: Process 25109 attached
[pid 25108] +++ exited with 0 +++
strace: Process 25110 attached
[pid 25109] +++ exited with 0 +++
strace: Process 25111 attached
[pid 25110] +++ exited with 0 +++
strace: Process 25112 attached
[pid 25111] +++ exited with 0 +++
[pid 25112] +++ exited with 0 +++
strace: Process 25113 attached
strace: Process 25114 attached
[pid 25113] +++ exited with 0 +++
[pid 25114] +++ exited with 0 +++
strace: Process 25115 attached
strace: Process 25116 attached
[pid 25115] +++ exited with 0 +++
strace: Process 25117 attached
[pid 25116] +++ exited with 0 +++
[pid 25117] +++ exited with 0 +++
strace: Process 25118 attached
strace: Process 25119 attached
[pid 25118] +++ exited with 0 +++
strace: Process 25120 attached
[pid 25119] +++ exited with 0 +++
strace: Process 25121 attached
[pid 25120] +++ exited with 0 +++
strace: Process 25122 attached
[pid 25121] +++ exited with 0 +++
[pid 25122] +++ exited with 0 +++
strace: Process 25123 attached
strace: Process 25124 attached
[pid 25123] +++ exited with 0 +++
strace: Process 25125 attached
[pid 25124] +++ exited with 0 +++
[pid 25125] +++ exited with 0 +++
[pid 24966] +++ exited with 0 +++
[pid 24967] +++ exited with 0 +++
[pid 24928] getrandom(0x2b4b07ab7494, 4, 0) = -1 ENOSYS (Function not implemented)
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__GI___arc4random_buf.part.0+0xd) [0x3d05d]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(arc4random+0x23) [0x3d103]
 > /scratch/user/bsferraz/sys/lib/libLLVM-15.so(_ZN4llvm3sys2fs16createUniquePathERKNS_5TwineERNS_15SmallVectorImplIcEEb+0xc4) [0xa7f684]
 > /scratch/user/bsferraz/sys/lib/libLLVM-15.so(_ZN4llvm3sys2fs8TempFile6createERKNS_5TwineEjNS1_9OpenFlagsE+0x85) [0xa802f5]
 > /scratch/user/bsferraz/sys/lib/libLLVM-15.so(_ZN4llvm12writeArchiveENS_9StringRefENS_8ArrayRefINS_16NewArchiveMemberEEEbNS_6object7Archive4KindEbbSt10unique_ptrINS_12MemoryBufferESt14default_deleteIS8_EE+0xa0) [0x27e8c00]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(LLVMRustWriteArchive+0x23a) [0x96daaa]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvXs_NtNtCsb6MjuUTC6RF_18rustc_codegen_llvm4back7archiveNtB4_18LlvmArchiveBuilderNtNtNtCsc69PRZpHr3K_17rustc_codegen_ssa4back7archive14ArchiveBuilder5build+0x8f0) [0x8c8ae0]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvNtNtCsc69PRZpHr3K_17rustc_codegen_ssa4back4link11link_binary+0x916) [0x1c01816]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvXs5_Csb6MjuUTC6RF_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCsc69PRZpHr3K_17rustc_codegen_ssa6traits7backend14CodegenBackend4link+0x24) [0x8a3ef4]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvMs1_NtCskAKw8pI6xjz_15rustc_interface7queriesNtB5_6Linker4link+0x3fd) [0x80615d]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvCslzmqIBn3a9T_10rustc_span15with_source_mapINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCINvNtCskAKw8pI6xjz_15rustc_interface9interface23create_compiler_and_runBJ_NCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0Es_0EB3q_+0x23b) [0x7169cb]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtCskAKw8pI6xjz_15rustc_interface9interface23create_compiler_and_runINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0EB2B_+0x3d3) [0x6ecb33]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMs_Csath8bPdUG0i_10scoped_tlsINtB5_9ScopedKeyNtCslzmqIBn3a9T_10rustc_span14SessionGlobalsE3setNCINvNtCskAKw8pI6xjz_15rustc_interface9interface12run_compilerINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0B2y_EB41_+0x62) [0x6e94d2]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtNtCs6Z0hJvZQRbe_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvNtCskAKw8pI6xjz_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB1m_9interface12run_compilerINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0B32_E0B32_EB4v_+0x79) [0x71bb79]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtCs6Z0hJvZQRbe_3std9panicking3tryINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedEINtNtNtBF_5panic11unwind_safe16AssertUnwindSafeNCNCINvMNtB4_6threadNtB2S_7Builder16spawn_unchecked_NCINvNtCskAKw8pI6xjz_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB3H_9interface12run_compilerBA_NCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0BA_E0BA_Es_00EEB5u_+0x2e) [0x69eede]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNSNvYNCINvMNtCs6Z0hJvZQRbe_3std6threadNtBa_7Builder16spawn_unchecked_NCINvNtCskAKw8pI6xjz_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB1d_9interface12run_compilerINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0B2T_E0B2T_Es_0INtNtNtB2Y_3ops8function6FnOnceuE9call_once6vtableB4m_+0x90) [0x6e9030]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN90_$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$9call_once17h75833ee271cfd292E+0x18) [0xad018]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN3std3sys4unix6thread6Thread3new12thread_start17hceab38778623fe14E+0x17) [0xb23c7]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(start_thread+0x2c4) [0x88e84]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__clone+0x40) [0x108390]
[pid 24928] getrandom(0x2b4b07ab7493, 5, 0) = -1 ENOSYS (Function not implemented)
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__GI___arc4random_buf.part.0+0xd) [0x3d05d]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(arc4random+0x23) [0x3d103]
 > /scratch/user/bsferraz/sys/lib/libLLVM-15.so(_ZN4llvm3sys2fs16createUniquePathERKNS_5TwineERNS_15SmallVectorImplIcEEb+0xc4) [0xa7f684]
 > /scratch/user/bsferraz/sys/lib/libLLVM-15.so(_ZN4llvm3sys2fs8TempFile6createERKNS_5TwineEjNS1_9OpenFlagsE+0x85) [0xa802f5]
 > /scratch/user/bsferraz/sys/lib/libLLVM-15.so(_ZN4llvm12writeArchiveENS_9StringRefENS_8ArrayRefINS_16NewArchiveMemberEEEbNS_6object7Archive4KindEbbSt10unique_ptrINS_12MemoryBufferESt14default_deleteIS8_EE+0xa0) [0x27e8c00]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(LLVMRustWriteArchive+0x23a) [0x96daaa]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvXs_NtNtCsb6MjuUTC6RF_18rustc_codegen_llvm4back7archiveNtB4_18LlvmArchiveBuilderNtNtNtCsc69PRZpHr3K_17rustc_codegen_ssa4back7archive14ArchiveBuilder5build+0x8f0) [0x8c8ae0]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvNtNtCsc69PRZpHr3K_17rustc_codegen_ssa4back4link11link_binary+0x916) [0x1c01816]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvXs5_Csb6MjuUTC6RF_18rustc_codegen_llvmNtB5_18LlvmCodegenBackendNtNtNtCsc69PRZpHr3K_17rustc_codegen_ssa6traits7backend14CodegenBackend4link+0x24) [0x8a3ef4]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNvMs1_NtCskAKw8pI6xjz_15rustc_interface7queriesNtB5_6Linker4link+0x3fd) [0x80615d]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvCslzmqIBn3a9T_10rustc_span15with_source_mapINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCINvNtCskAKw8pI6xjz_15rustc_interface9interface23create_compiler_and_runBJ_NCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0Es_0EB3q_+0x23b) [0x7169cb]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtCskAKw8pI6xjz_15rustc_interface9interface23create_compiler_and_runINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0EB2B_+0x3d3) [0x6ecb33]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvMs_Csath8bPdUG0i_10scoped_tlsINtB5_9ScopedKeyNtCslzmqIBn3a9T_10rustc_span14SessionGlobalsE3setNCINvNtCskAKw8pI6xjz_15rustc_interface9interface12run_compilerINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0B2y_EB41_+0x62) [0x6e94d2]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtNtCs6Z0hJvZQRbe_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvNtCskAKw8pI6xjz_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB1m_9interface12run_compilerINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0B32_E0B32_EB4v_+0x79) [0x71bb79]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RINvNtCs6Z0hJvZQRbe_3std9panicking3tryINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedEINtNtNtBF_5panic11unwind_safe16AssertUnwindSafeNCNCINvMNtB4_6threadNtB2S_7Builder16spawn_unchecked_NCINvNtCskAKw8pI6xjz_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB3H_9interface12run_compilerBA_NCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0BA_E0BA_Es_00EEB5u_+0x2e) [0x69eede]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-2eaefa8cde0b625e.so(_RNSNvYNCINvMNtCs6Z0hJvZQRbe_3std6threadNtBa_7Builder16spawn_unchecked_NCINvNtCskAKw8pI6xjz_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB1d_9interface12run_compilerINtNtCs9ZTcLekmlwh_4core6result6ResultuNtCsjNamqeRrLib_12rustc_errors15ErrorGuaranteedENCNvCsgtDLS7mlP0r_12rustc_driver12run_compilers_0E0B2T_E0B2T_Es_0INtNtNtB2Y_3ops8function6FnOnceuE9call_once6vtableB4m_+0x90) [0x6e9030]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN90_$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$9call_once17h75833ee271cfd292E+0x18) [0xad018]
 > /scratch/user/bsferraz/sys/install/builds/rustc-1.64.0-src/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-fbc4878b4cd5f717.so(_ZN3std3sys4unix6thread6Thread3new12thread_start17hceab38778623fe14E+0x17) [0xb23c7]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(start_thread+0x2c4) [0x88e84]
 > /scratch/user/bsferraz/sys/lib/libc.so.6(__clone+0x40) [0x108390]
