
---- [run-make] run-make/fmt-write-bloat stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/git/rust/build/x86_64-unknown-linux-gnu/test/run-make/fmt-write-bloat/fmt-write-bloat:/git/rust/build/x86_64-unknown-linux-gnu/stage1/lib:/git/rust/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/git/rust/build/x86_64-unknown-linux-gnu/stage0/lib" '/git/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc' --out-dir /git/rust/build/x86_64-unknown-linux-gnu/test/run-make/fmt-write-bloat/fmt-write-bloat -L /git/rust/build/x86_64-unknown-linux-gnu/test/run-make/fmt-write-bloat/fmt-write-bloat  main.rs -O
nm /git/rust/build/x86_64-unknown-linux-gnu/test/run-make/fmt-write-bloat/fmt-write-bloat/main | "/git/rust/src/etc/cat-and-grep.sh" -v panicking panic_fmt panic_bounds_check pad_integral Display Debug
[[[ begin stdout ]]]
00000000000020b2 R anon.ad56e829de00bb5e05e79e94a78aaeca.0.llvm.11246502207772803501
0000000000004008 B __bss_start
0000000000004008 b completed.8060
                 w __cxa_finalize@@GLIBC_2.2.5
0000000000001070 t deregister_tm_clones
00000000000010e0 t __do_global_dtors_aux
0000000000003d20 d __do_global_dtors_aux_fini_array_entry
0000000000004000 D __dso_handle
0000000000003dd8 d _DYNAMIC
0000000000004008 D _edata
0000000000004010 B _end
0000000000001d6c T _fini
0000000000001120 t frame_dummy
0000000000003d18 d __frame_dummy_init_array_entry
0000000000002524 r __FRAME_END__
0000000000003f98 d _GLOBAL_OFFSET_TABLE_
                 w __gmon_start__
000000000000217c r __GNU_EH_FRAME_HDR
0000000000001000 t _init
0000000000003d20 d __init_array_end
0000000000003d18 d __init_array_start
                 w _ITM_deregisterTMCloneTable
                 w _ITM_registerTMCloneTable
0000000000001290 T __libc_csu_fini
0000000000001220 T __libc_csu_init
                 U __libc_start_main@@GLIBC_2.2.5
00000000000011c0 T main
00000000000010a0 t register_tm_clones
00000000000011b0 T rust_begin_unwind
0000000000001040 T _start
0000000000004008 D __TMC_END__
0000000000001d60 T _ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hec09389da73b5e7cE
0000000000001c30 T _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h54377f781f7a5c8fE
0000000000001c30 T _ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h4fa1341a9dac6525E
00000000000012c0 T _ZN4core3fmt5write17h100dc89493224d93E
0000000000001a80 t _ZN4core3fmt9Formatter12pad_integral12write_prefix17h204c271bcefaaa8cE
00000000000015b0 T _ZN4core3fmt9Formatter12pad_integral17hb075f5592a5d8a70E
00000000000012a0 T _ZN4core3ops8function6FnOnce9call_once17hd1e4d320fcc785faE.llvm.14856878211942904523
0000000000001130 t _ZN4core3ptr13drop_in_place17h8d4e61ec59d31a0eE
0000000000001ad0 t _ZN4core3ptr13drop_in_place17hd5cc673701fb5dfbE
0000000000001ba0 t _ZN4core4iter8adapters3zip16Zip$LT$A$C$B$GT$3new17h5f4b1cffaf34d111E
0000000000001be0 t _ZN4core4iter8adapters3zip16Zip$LT$A$C$B$GT$3new17hf19b0aefdc63476aE
0000000000001ae0 T _ZN4core9panicking18panic_bounds_check17ha31d9d2fa83bb84dE
0000000000001b60 T _ZN4core9panicking9panic_fmt17he9332856f0d02b11E
0000000000001140 t _ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h3bc2d3f42fbd7666E
0000000000001150 t _ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h0fd2e1dfd6fb1e2aE
00000000000011a0 t _ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h46dc1af754709fa5E

[[[ end stdout ]]]
Error: should not match: panicking
Error: should not match: panic_fmt
Error: should not match: panic_bounds_check
Error: should not match: pad_integral
Error: should not match: Display
