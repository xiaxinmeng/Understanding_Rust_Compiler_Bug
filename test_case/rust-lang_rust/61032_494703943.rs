plain
[00:47:18]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:47:44] error: failed to run custom build command for `rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)`
[00:47:44] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_lsan-ede59fccead25b1f/build-script-build` (exit code: 101)
[00:47:44] --- stdout
[00:47:44] cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
[00:47:44] cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/lit.common.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/test-darwin-interface.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-signed-integer-truncation.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-unsigned-integer-truncation.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-signed-integer-truncation-or-sign-change.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/recover-dedup-limit.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-integer-sign-change.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/alignment-assumption.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/recover-dedup.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/uadd-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/CMakeLists.txt
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lit.common.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattisf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulosi4_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/endianness.h
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/unordsf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatundixf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunstfsi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powixf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divsc3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ltsf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/unorddf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ucmpdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixtfti_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subvdi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subsf3vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashldi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatsidfvfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/comparesf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/riscv/mulsi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/letf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ffsdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncdfhf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lshrti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divdi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/multi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/trunctfsf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/parityti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntisf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/cpu_model_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divxc3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divdc3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunssisfvfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negvsi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divsf3vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subdf3vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/test
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/bswapsi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negsf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/popcountsi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattidf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunstfti_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunditf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ctzdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addsf3vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulsc3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/umodsi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivsi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/absvti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfsi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extendsftf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfsivfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntidf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/eqtf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clzti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/netf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/unordtf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divtf3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/popcountdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatsisfvfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ctzti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/getf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashrdi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/cmpti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatditf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattitf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/eqsf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulsf3vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/trampoline_setup_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfti_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulodi4_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gedf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunstfdi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/absvsi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/modti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divtc3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powitf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/umoddi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ledf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfti_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addvti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncdfsf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clzsi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixsfdi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/eqdf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lttf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatdidf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/modsi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ctzsi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/nesf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatundidf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lesf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/compiler_rt_logbl_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fp_test.h
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/multf3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatdisf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfdi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashrti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/cmpdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gttf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negvti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gtsf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/nedf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muldf3vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/adddf3vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntitf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatsitf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattixf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gcc_personality_test_helper.cxx
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gcc_personality_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subtf3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ucmpti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncdfsf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extenddftf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixxfti_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lshrdi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gesf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashlti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addtf3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfsivfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/paritysi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extebdsfdf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatundisf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subvsi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negdf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divmodsi4_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/trunctfdf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divdf3vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muldc3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/absvdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subvti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divsi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/multc3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powidf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulvti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/compiler_rt_logb_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negvdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixtfsi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/paritydi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivmodti4_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixdfti_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulxc3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/compiler_rt_logbf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extendhfsf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixsfti_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixdfdi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixxfdi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/bswapdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muloti4_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/moddi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivmoddi4_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/test
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/fixunstfti_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatunditf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qadd_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/fixunstfti_test.h
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatditf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floattitf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qsub_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/DD.h
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatditf_test.h
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatunditf_test.h
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qmul_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/fixtfdi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floattitf_test.h
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qdiv_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addvsi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfsi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntixf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulvdi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ffsti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixsfsivfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lit.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muldi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addvdi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/enable_execute_stack_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivdi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsxfti_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clzdi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsxfsi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cfcmpeq_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/call_apsr.h
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_drsub_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cdcmple_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/call_apsr.S
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_uidivmod_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cfcmple_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_idivmod_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_frsub_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cdcmpeq_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_uldivmod_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ffssi2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatdixf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfdi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixtfdi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gtdf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunsitf_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powisf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/popcountti2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulvsi3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clear_cache_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsxfdi_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ltdf2vfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivmodsi4_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/umodti3_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/comparedf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncsfhf2_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixdfsivfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunssidfvfp_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/timing.h
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatdisf.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatdidf.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/lshrdi3.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/moddi3.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/muldi3.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatundidf.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/modsi3.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatundixf.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/time
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/negdi2.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/ashrdi3.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatdixf.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/udivdi3.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/umoddi3.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/ashldi3.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/divdi3.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatundisf.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/TestCases/Darwin/os_version_check_test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/TestCases/Darwin/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/TestCases/Darwin/os_version_check_test_no_core_foundation.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/CMakeLists.txt
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/lit.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/lit.common.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/high_allocator_contention.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Darwin/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Darwin/dispatch.mm
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/suppressions_file.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/sanity_check_pure_c.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/recoverable_leak_check.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_globals_uninitialized.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/many_tls_keys.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_unaligned.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/pointer_to_self.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/leak_check_before_thread_started.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/leak_check_at_exit.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/register_root_region.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_stacks_threaded.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/new_array_with_dtor_0.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Posix/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/disabler.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/default_options.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/stale_stack_leak.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_registers.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_stacks.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_globals_initialized.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_after_return.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/ignore_object_errors.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/do_leak_check_override.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/strace_test.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/large_allocation_leak.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/swapcontext.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/ignore_object.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/disabler.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/use_tls_pthread_specific_dynamic.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/fork_and_leak.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/use_tls_dynamic.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/fork_threaded.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/disabler_in_tsd_destructor.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/guard-page.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/fork.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/use_tls_pthread_specific_static.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/use_tls_static.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/log-path_test.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/cleanup_in_tsd_destructor.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/suppressions_default.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/print_suppressions.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/link_turned_off.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_poisoned_asan.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/CMakeLists.txt
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/lit.common.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Float/cast-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/Function/function.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/Function/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr-virtual-base.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr-corrupted-vtable-itanium.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/null.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/misaligned.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/PR33221.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr-virtual-base-construction.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr-non-unique-typeinfo.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/Linux/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/Linux/PR33221.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/monitor.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/builtins.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/nonnull-arg.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/coverage-levels.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/missing_return.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/no-interception.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/nonnull.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/vla.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/bool.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/bool.m
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/unreachable.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/enum.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/log-path_test.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/deduplication.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/bounds.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Linux/ubsan_options.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Linux/print_stack_trace.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Linux/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/nullability.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Inputs/no-interception-dso.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Inputs/returns-unexpectedly.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/unsigned-integer-truncation-blacklist.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-sign-change.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-arithmetic-value-change.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-conversion.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/unsigned-integer-truncation.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-truncation.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/unsigned-integer-truncation-summary.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation-or-sign-change-blacklist.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation-blacklist.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation-or-sign-change-summary.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-sign-change-blacklist.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation-summary.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-sign-change-summary.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-assume_aligned-on-function.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-builtin_assume_aligned-two-params.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/index-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-assume_aligned-on-function-two-params.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/unsigned-index-expression.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-openmp.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-blacklist.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-align_value-on-lvalue.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-summary.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-align_value-on-paramvar.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-alloc_align-on-function.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-builtin_assume_aligned-three-params.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-builtin_assume_aligned-three-params-variable.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-alloc_align-on-function-variable.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/incdec-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/summary.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/umul-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/suppressions.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/no-recover.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/sub-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/add-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/div-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/shift.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/usub-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/mul-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/uincdec-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/negate-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/div-zero.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/uadd-overflow.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan/CMakeLists.txt
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/heap-buffer-overflow.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/abort-message-android.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/double-free.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/check-interface.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/random-align-right.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/sizes.cpp
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/longjmp.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/realloc-test.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/pthread_exit.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/halt-on-error.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/realloc-after-free.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/print-memory-usage.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/print-memory-usage-android.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/malloc_fill.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/malloc-test.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/use-after-free.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/many-threads-uaf.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/cfi.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/tail-magic.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/thread-uaf.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/uaf_with_rb_distance.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/mem-intrinsics.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/allocator_returns_null.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/Posix/system-allocator-fallback.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/Posix/posix_memalign-alignment.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/Posix/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/new-test.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/stack-oob.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/hwasan-print-shadow.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/sanitizer_malloc.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/stack-uar.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/stack-history-length.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/deep-recursion.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/Linux/pvalloc-overflow.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/Linux/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/Linux/aligned_alloc-alignment.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/rich-stack.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/TestCases/mem-intrinsics-zero-size.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/CMakeLists.txt
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/hwasan/lit.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/Unit/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/basic-filtering.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-mode-multiple.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/patching-unpatching.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/custom-event-handler-alignment.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/c-test.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/custom-event-logging.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/logging-modes.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/argv0-log-file-name.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/func-id-utils.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-single-thread.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/common-trampoline-alignment.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fixedsize-logging.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/coverage-sample.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/profiling-multi-threaded.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/clang-no-xray-instrument.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-reinit.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/pic_test.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-mode.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/arg1-logger.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-thread-order.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/quiet-start.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-mode-inmemory.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/arg1-logging-implicit-this.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fork_basic_logging.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/optional-inmemory-log.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/arg1-arg0-logging.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/always-never-instrument.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/profiling-single-threaded.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/CMakeLists.txt
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/xray/lit.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/lit.common.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/Unit/lit.site.cfg.in
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/sanitizer_coverage_trace_pc_guard-init.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/Darwin/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/Darwin/print-stack-trace.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/Darwin/abort_on_error.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/symbolize_pc.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/symbolize_pc_inline.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/strnlen.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/symbolize_stack.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/strstr.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/strspn.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/printf-ldbl.c
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/gid_from_group.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/group_from_gid.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/fparseln.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/sha2.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/paccept.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/netent.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/strmode.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/user_from_uid.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/faccessat.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/lit.local.cfg
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/md5.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/protoent.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/funopen2.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/statvfs1.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/md2.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/getgroupmembership.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/mi_vector_hash.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/rmd160.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/ttyent.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/getvfsstat.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/cdb.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/uid_from_user.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/strtoi.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/sysctlgetmibinfo.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/asysctl.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/sha1.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/md4.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/NetBSD/getgrouplist.cc
[00:47:44] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/sanitizer_common/TestCases/malloc_hook.cc
---
[00:47:47] -- Detecting CXX compiler ABI info
[00:47:47] -- Detecting CXX compiler ABI info - done
[00:47:47] -- Detecting CXX compile features
[00:47:47] -- Detecting CXX compile features - done
[00:47:47] -- Looking for unwind.h
[00:47:47] -- Looking for unwind.h - found
[00:47:47] -- Looking for rpc/xdr.h
[00:47:47] -- Looking for rpc/xdr.h - found
[00:47:47] -- Looking for fopen in c
[00:47:47] -- Looking for fopen in c - found
[00:47:47] -- Looking for __gcc_personality_v0 in gcc_s
[00:47:47] -- Looking for __gcc_personality_v0 in gcc_s - found
[00:47:47] -- Looking for __gcc_personality_v0 in gcc_s - found
[00:47:47] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[00:47:47] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_G_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Success
[00:47:47] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[00:47:47] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[00:47:47] -- Looking for __func__
[00:47:47] -- Looking for __func__ - found
[00:47:47] -- Looking for dlopen in dl - found
[00:47:47] -- Looking for shm_open in rt
[00:47:47] -- Looking for shm_open in rt - found
[00:47:47] -- Looking for pow in m
[00:47:47] -- Looking for pow in m
[00:47:47] -- Looking for pow in m - found
[00:47:47] -- Looking for pthread_create in pthread
[00:47:47] -- Looking for pthread_create in pthread - found
[00:47:47] -- Looking for backtrace in execinfo
[00:47:47] -- Looking for backtrace in execinfo - not found
[00:47:47] -- Looking for __cxa_throw in c++
[00:47:47] -- Looking for __cxa_throw in c++ - not found
[00:47:47] -- Looking for __cxa_throw in stdc++
[00:47:47] -- Looking for __cxa_throw in stdc++ - found
[00:47:47] -- Looking for __i386__
[00:47:47] -- Looking for __i386__ - found
[00:47:47] -- Compiler-RT supported architectures: x86_64;i386
[00:47:47] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[00:47:47] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[00:47:47] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[00:47:47] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[00:47:47] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[00:47:47] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[00:47:47] -- Performing Test HAS_THREAD_LOCAL - Success
[00:47:47] -- Configuring done
[00:47:47] -- Generating done
[00:47:47] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/lsan/build
[00:47:47] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/lsan/build
[00:47:47] running: "cmake" "--build" "." "--target" "clang_rt.lsan-x86_64" "--config" "Release" "--"
[00:47:47] Scanning dependencies of target RTLSanCommon.x86_64
[00:47:47] [  0%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common.cc.o
[00:47:47] [  9%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common_linux.cc.o
[00:47:47] [  9%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common_mac.cc.o
[00:47:47] [  9%] Built target RTLSanCommon.x86_64
[00:47:47] Scanning dependencies of target RTSanitizerCommonCoverage.x86_64
[00:47:47] [  9%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sancov_flags.cc.o
[00:47:47] [  9%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[00:47:47] [  9%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[00:47:47] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[00:47:47] [ 18%] Built target RTSanitizerCommonCoverage.x86_64
[00:47:47] Scanning dependencies of target RTSanitizerCommon.x86_64
[00:47:47] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[00:47:47] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[00:47:47] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[00:47:47] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[00:47:47] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[00:47:47] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[00:47:47] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[00:47:47] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[00:47:47] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[00:47:47] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[00:47:47] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_netbsd.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_openbsd.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_freebsd.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[00:47:47] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_openbsd.cc.o
[00:47:47] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[00:47:47] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_solaris.cc.o
[00:47:47] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_posix.cc.o
[00:47:47] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_printf.cc.o
[00:47:47] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_common.cc.o
[00:47:47] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_bsd.cc.o
[00:47:47] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_linux.cc.o
[00:47:47] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_mac.cc.o
[00:47:47] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_solaris.cc.o
[00:47:47] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_rtems.cc.o
[00:47:47] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_solaris.cc.o
[00:47:47] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_stoptheworld_mac.cc.o
[00:47:47] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_suppressions.cc.o
[00:47:47] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_tls_get_addr.cc.o
[00:47:47] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_thread_registry.cc.o
[00:47:48] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_type_traits.cc.o
[00:47:48] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_win.cc.o
[00:47:48] [ 54%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_x86_64.S.o
[00:47:48] [ 63%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_mips64.S.o
[00:47:48] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_termination.cc.o
[00:47:48] [ 63%] Built target RTSanitizerCommon.x86_64
[00:47:48] Scanning dependencies of target RTSanitizerCommonLibc.x86_64
[00:47:48] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cc.o
[00:47:48] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_allocator_checks.cc.o
[00:47:48] [ 72%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o
[00:47:48] --- stderr
[00:47:48] --- stderr
[00:47:48] /checkout/src/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_linux_libcdep.cc:706:10: error: use of undeclared identifier 'CPU_COUNT'
[00:47:48]   return CPU_COUNT(&CPUs);
[00:47:48] 1 error generated.
[00:47:48] 1 error generated.
[00:47:48] gmake[3]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o] Error 1
[00:47:48] gmake[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/all] Error 2
[00:47:48] gmake[1]: *** [lib/lsan/CMakeFiles/clang_rt.lsan-x86_64.dir/rule] Error 2
[00:47:48] gmake: *** [clang_rt.lsan-x86_64] Error 2
[00:47:48] command did not execute successfully, got: exit code: 2
[00:47:48] 
[00:47:48] 
[00:47:48] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
[00:47:48] 
[00:47:48] warning: build failed, waiting for other jobs to finish...
[00:47:48] error: failed to run custom build command for `rustc_msan v0.0.0 (/checkout/src/librustc_msan)`
[00:47:48] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_msan-5edcba0508c60850/build-script-build` (exit code: 101)
[00:47:48] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_msan-5edcba0508c60850/build-script-build` (exit code: 101)
[00:47:48] --- stdout
[00:47:48] cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
[00:47:48] cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/lit.common.cfg
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/lit.site.cfg.in
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/test-darwin-interface.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-signed-integer-truncation.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-unsigned-integer-truncation.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-signed-integer-truncation-or-sign-change.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/recover-dedup-limit.cpp
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-integer-sign-change.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/alignment-assumption.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/recover-dedup.cpp
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/uadd-overflow.cpp
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/CMakeLists.txt
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lit.common.cfg
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/lit.site.cfg.in
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattisf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulosi4_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/endianness.h
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/unordsf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatundixf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunstfsi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powixf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divsc3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ltsf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/unorddf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ucmpdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixtfti_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subvdi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subsf3vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashldi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatsidfvfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/comparesf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/riscv/mulsi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/letf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ffsdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncdfhf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lshrti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divdi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/multi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/trunctfsf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/parityti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntisf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/cpu_model_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divxc3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divdc3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunssisfvfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negvsi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divsf3vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subdf3vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/test
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/bswapsi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negsf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/popcountsi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattidf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunstfti_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunditf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ctzdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addsf3vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulsc3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/umodsi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivsi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/absvti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfsi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extendsftf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lit.site.cfg.in
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfsivfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntidf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/eqtf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clzti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/netf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/unordtf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divtf3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/popcountdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatsisfvfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ctzti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/getf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashrdi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/cmpti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatditf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattitf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/eqsf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulsf3vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/trampoline_setup_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfti_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulodi4_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gedf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunstfdi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/absvsi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/modti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divtc3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powitf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/umoddi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ledf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfti_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addvti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncdfsf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clzsi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixsfdi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/eqdf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lttf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatdidf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/modsi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ctzsi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/nesf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatundidf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lesf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/compiler_rt_logbl_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fp_test.h
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/multf3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatdisf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfdi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashrti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/cmpdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gttf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negvti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gtsf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/nedf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muldf3vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/adddf3vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntitf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatsitf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattixf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gcc_personality_test_helper.cxx
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gcc_personality_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subtf3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ucmpti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncdfsf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extenddftf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixxfti_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lshrdi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gesf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashlti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addtf3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfsivfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/paritysi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extebdsfdf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatundisf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subvsi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negdf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divmodsi4_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/trunctfdf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divdf3vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muldc3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/absvdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subvti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divsi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/multc3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powidf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulvti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/compiler_rt_logb_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negvdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixtfsi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/paritydi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivmodti4_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixdfti_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulxc3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/compiler_rt_logbf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extendhfsf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixsfti_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixdfdi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixxfdi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/bswapdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muloti4_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/moddi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivmoddi4_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/test
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/fixunstfti_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatunditf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qadd_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/fixunstfti_test.h
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatditf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floattitf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qsub_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/DD.h
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatditf_test.h
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatunditf_test.h
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qmul_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/fixtfdi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floattitf_test.h
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qdiv_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addvsi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfsi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntixf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulvdi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ffsti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixsfsivfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lit.cfg
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muldi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addvdi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/enable_execute_stack_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivdi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsxfti_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clzdi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsxfsi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cfcmpeq_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/call_apsr.h
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_drsub_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cdcmple_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/call_apsr.S
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_uidivmod_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cfcmple_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_idivmod_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_frsub_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cdcmpeq_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_uldivmod_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ffssi2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatdixf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfdi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixtfdi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gtdf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunsitf_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powisf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/popcountti2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulvsi3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clear_cache_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsxfdi_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ltdf2vfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivmodsi4_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/umodti3_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/comparedf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncsfhf2_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixdfsivfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunssidfvfp_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/timing.h
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatdisf.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatdidf.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/lshrdi3.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/moddi3.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/muldi3.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatundidf.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/modsi3.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatundixf.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/time
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/negdi2.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/ashrdi3.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatdixf.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/udivdi3.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/umoddi3.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/ashldi3.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/divdi3.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatundisf.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/TestCases/Darwin/os_version_check_test.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/TestCases/Darwin/lit.local.cfg
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/TestCases/Darwin/os_version_check_test_no_core_foundation.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/CMakeLists.txt
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/lit.cfg
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/lit.common.cfg
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/lit.site.cfg.in
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/high_allocator_contention.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Darwin/lit.local.cfg
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Darwin/dispatch.mm
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/suppressions_file.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/sanity_check_pure_c.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/recoverable_leak_check.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_globals_uninitialized.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/many_tls_keys.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_unaligned.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/pointer_to_self.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/leak_check_before_thread_started.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/leak_check_at_exit.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/register_root_region.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_stacks_threaded.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/new_array_with_dtor_0.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Posix/lit.local.cfg
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/disabler.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/default_options.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/stale_stack_leak.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_registers.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_stacks.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_globals_initialized.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_after_return.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/ignore_object_errors.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/do_leak_check_override.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/strace_test.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/large_allocation_leak.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/swapcontext.cc
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/ignore_object.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/disabler.c
[00:47:48] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Linux/use_tls_pthread_specific_dynamic.cc
---
[00:47:51] -- Detecting CXX compiler ABI info
[00:47:51] -- Detecting CXX compiler ABI info - done
[00:47:51] -- Detecting CXX compile features
[00:47:51] -- Detecting CXX compile features - done
[00:47:51] -- Looking for unwind.h
[00:47:51] -- Looking for unwind.h - found
[00:47:51] -- Looking for rpc/xdr.h
[00:47:51] -- Looking for rpc/xdr.h - found
[00:47:51] -- Looking for fopen in c
[00:47:51] -- Looking for fopen in c - found
[00:47:51] -- Looking for __gcc_personality_v0 in gcc_s
[00:47:51] -- Looking for __gcc_personality_v0 in gcc_s - found
[00:47:51] -- Looking for __gcc_personality_v0 in gcc_s - found
[00:47:51] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[00:47:51] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_G_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Success
[00:47:51] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[00:47:51] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[00:47:51] -- Looking for __func__
[00:47:51] -- Looking for __func__ - found
[00:47:51] -- Looking for dlopen in dl - found
[00:47:51] -- Looking for shm_open in rt
[00:47:51] -- Looking for shm_open in rt - found
[00:47:51] -- Looking for pow in m
[00:47:51] -- Looking for pow in m
[00:47:51] -- Looking for pow in m - found
[00:47:51] -- Looking for pthread_create in pthread
[00:47:51] -- Looking for pthread_create in pthread - found
[00:47:51] -- Looking for backtrace in execinfo
[00:47:51] -- Looking for backtrace in execinfo - not found
[00:47:51] -- Looking for __cxa_throw in c++
[00:47:51] -- Looking for __cxa_throw in c++ - not found
[00:47:51] -- Looking for __cxa_throw in stdc++
[00:47:51] -- Looking for __cxa_throw in stdc++ - found
[00:47:51] -- Looking for __i386__
[00:47:51] -- Looking for __i386__ - found
[00:47:51] -- Compiler-RT supported architectures: x86_64;i386
[00:47:51] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[00:47:51] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[00:47:51] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[00:47:51] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[00:47:51] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[00:47:51] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[00:47:51] -- Performing Test HAS_THREAD_LOCAL - Success
[00:47:51] -- Configuring done
[00:47:51] -- Generating done
[00:47:51] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/msan/build
[00:47:51] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/msan/build
[00:47:51] running: "cmake" "--build" "." "--target" "clang_rt.msan-x86_64" "--config" "Release" "--"
[00:47:51] Scanning dependencies of target RTUbsan.x86_64
[00:47:51] [  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_diag.cc.o
[00:47:51] [  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_init.cc.o
[00:47:51] [  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_flags.cc.o
[00:47:51] [  9%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_handlers.cc.o
[00:47:51] [  9%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_monitor.cc.o
[00:47:51] [  9%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_value.cc.o
[00:47:51] [  9%] Built target RTUbsan.x86_64
[00:47:51] Scanning dependencies of target RTSanitizerCommonCoverage.x86_64
[00:47:51] [  9%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sancov_flags.cc.o
[00:47:51] [  9%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[00:47:51] [  9%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[00:47:51] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[00:47:51] [ 18%] Built target RTSanitizerCommonCoverage.x86_64
[00:47:51] Scanning dependencies of target RTSanitizerCommon.x86_64
[00:47:51] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[00:47:51] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[00:47:51] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[00:47:51] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[00:47:51] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[00:47:51] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[00:47:51] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[00:47:51] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[00:47:51] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[00:47:51] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[00:47:51] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_netbsd.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_openbsd.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_freebsd.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[00:47:51] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_openbsd.cc.o
[00:47:51] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[00:47:51] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_solaris.cc.o
[00:47:51] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_posix.cc.o
[00:47:51] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_printf.cc.o
[00:47:51] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_common.cc.o
[00:47:51] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_bsd.cc.o
[00:47:51] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_linux.cc.o
[00:47:51] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_mac.cc.o
[00:47:51] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_solaris.cc.o
[00:47:51] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_rtems.cc.o
[00:47:51] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_solaris.cc.o
[00:47:51] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_stoptheworld_mac.cc.o
[00:47:51] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_suppressions.cc.o
[00:47:51] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_tls_get_addr.cc.o
[00:47:51] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_thread_registry.cc.o
[00:47:51] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_type_traits.cc.o
[00:47:51] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_win.cc.o
[00:47:51] [ 54%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_x86_64.S.o
[00:47:51] [ 63%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_mips64.S.o
[00:47:51] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_termination.cc.o
[00:47:51] [ 63%] Built target RTSanitizerCommon.x86_64
[00:47:51] Scanning dependencies of target RTSanitizerCommonLibc.x86_64
[00:47:51] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cc.o
[00:47:51] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_allocator_checks.cc.o
[00:47:51] [ 72%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o
[00:47:51] --- stderr
[00:47:51] --- stderr
[00:47:51] /checkout/src/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_linux_libcdep.cc:706:10: error: use of undeclared identifier 'CPU_COUNT'
[00:47:51]   return CPU_COUNT(&CPUs);
[00:47:51] 1 error generated.
[00:47:51] 1 error generated.
[00:47:51] gmake[3]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o] Error 1
[00:47:51] gmake[3]: *** Waiting for unfinished jobs....
[00:47:51] gmake[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/all] Error 2
[00:47:51] gmake[1]: *** [lib/msan/CMakeFiles/clang_rt.msan-x86_64.dir/rule] Error 2
[00:47:51] gmake: *** [clang_rt.msan-x86_64] Error 2
[00:47:51] command did not execute successfully, got: exit code: 2
[00:47:51] 
[00:47:51] 
[00:47:51] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
[00:47:51] 
[00:47:51] warning: build failed, waiting for other jobs to finish...
[00:47:51] error: failed to run custom build command for `rustc_asan v0.0.0 (/checkout/src/librustc_asan)`
[00:47:51] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_asan-51911bfdc32db5dc/build-script-build` (exit code: 101)
[00:47:51] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_asan-51911bfdc32db5dc/build-script-build` (exit code: 101)
[00:47:51] --- stdout
[00:47:51] cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
[00:47:51] cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/lit.common.cfg
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/lit.site.cfg.in
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/test-darwin-interface.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-signed-integer-truncation.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-unsigned-integer-truncation.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-signed-integer-truncation-or-sign-change.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/recover-dedup-limit.cpp
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-integer-sign-change.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/alignment-assumption.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/recover-dedup.cpp
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/uadd-overflow.cpp
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/ubsan_minimal/CMakeLists.txt
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lit.common.cfg
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/lit.site.cfg.in
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattisf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulosi4_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/endianness.h
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/unordsf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatundixf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunstfsi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powixf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divsc3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ltsf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/unorddf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ucmpdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixtfti_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subvdi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subsf3vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashldi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatsidfvfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/comparesf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/riscv/mulsi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/letf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ffsdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncdfhf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lshrti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divdi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/multi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/trunctfsf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/parityti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntisf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/cpu_model_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divxc3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divdc3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunssisfvfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negvsi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divsf3vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subdf3vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/test
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/bswapsi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negsf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/popcountsi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattidf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunstfti_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunditf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ctzdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addsf3vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulsc3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/umodsi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivsi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/absvti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfsi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extendsftf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lit.site.cfg.in
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfsivfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntidf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/eqtf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clzti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/netf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/unordtf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divtf3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/popcountdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatsisfvfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ctzti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/getf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashrdi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/cmpti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatditf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattitf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/eqsf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulsf3vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/trampoline_setup_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfti_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulodi4_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gedf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunstfdi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/absvsi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/modti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divtc3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powitf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/umoddi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ledf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfti_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addvti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncdfsf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clzsi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixsfdi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/eqdf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lttf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatdidf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/modsi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ctzsi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/nesf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatundidf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lesf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/compiler_rt_logbl_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fp_test.h
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/multf3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatdisf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfdi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashrti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/cmpdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gttf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negvti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gtsf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/nedf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muldf3vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/adddf3vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntitf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatsitf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floattixf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gcc_personality_test_helper.cxx
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gcc_personality_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subtf3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ucmpti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncdfsf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extenddftf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixxfti_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lshrdi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gesf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ashlti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addtf3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfsivfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/paritysi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extebdsfdf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatundisf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subvsi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negdf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divmodsi4_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/trunctfdf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divdf3vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muldc3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/absvdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/subvti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/divsi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/multc3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powidf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulvti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/compiler_rt_logb_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/negvdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixtfsi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/paritydi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivmodti4_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixdfti_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulxc3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/compiler_rt_logbf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/extendhfsf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixsfti_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixdfdi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixxfdi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/bswapdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muloti4_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/moddi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivmoddi4_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/test
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/fixunstfti_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatunditf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qadd_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/fixunstfti_test.h
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatditf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floattitf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qsub_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/DD.h
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatditf_test.h
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floatunditf_test.h
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qmul_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/fixtfdi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/floattitf_test.h
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ppc/qdiv_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addvsi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunssfsi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatuntixf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulvdi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ffsti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixsfsivfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/lit.cfg
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/muldi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/addvdi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/enable_execute_stack_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivdi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsxfti_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clzdi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsxfsi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cfcmpeq_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/call_apsr.h
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_drsub_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cdcmple_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/call_apsr.S
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_uidivmod_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cfcmple_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_idivmod_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_frsub_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_cdcmpeq_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/arm/aeabi_uldivmod_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ffssi2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatdixf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsdfdi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixtfdi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/gtdf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunsitf_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/powisf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/popcountti2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/mulvsi3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/clear_cache_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixunsxfdi_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/ltdf2vfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/udivmodsi4_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/umodti3_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/comparedf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/truncsfhf2_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/fixdfsivfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/Unit/floatunssidfvfp_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/timing.h
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatdisf.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatdidf.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/lshrdi3.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/moddi3.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/muldi3.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatundidf.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/modsi3.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatundixf.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/time
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/negdi2.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/ashrdi3.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatdixf.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/udivdi3.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/umoddi3.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/ashldi3.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/divdi3.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/timing/floatundisf.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/TestCases/Darwin/os_version_check_test.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/TestCases/Darwin/lit.local.cfg
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/TestCases/Darwin/os_version_check_test_no_core_foundation.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/CMakeLists.txt
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/builtins/lit.cfg
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/lit.common.cfg
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/lit.site.cfg.in
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/high_allocator_contention.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Darwin/lit.local.cfg
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Darwin/dispatch.mm
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/suppressions_file.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/sanity_check_pure_c.c
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/recoverable_leak_check.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_globals_uninitialized.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/many_tls_keys.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_unaligned.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/pointer_to_self.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/leak_check_before_thread_started.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/leak_check_at_exit.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/register_root_region.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_stacks_threaded.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/new_array_with_dtor_0.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/Posix/lit.local.cfg
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/disabler.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/default_options.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/stale_stack_leak.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_registers.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_stacks.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_globals_initialized.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/use_after_return.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/ignore_object_errors.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/do_leak_check_override.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/strace_test.cc
[00:47:51] cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/test/lsan/TestCases/large_allocation_leak.cc
---
[00:47:54] -- Detecting CXX compiler ABI info
[00:47:54] -- Detecting CXX compiler ABI info - done
[00:47:54] -- Detecting CXX compile features
[00:47:54] -- Detecting CXX compile features - done
[00:47:54] -- Looking for unwind.h
[00:47:54] -- Looking for unwind.h - found
[00:47:54] -- Looking for rpc/xdr.h
[00:47:54] -- Looking for rpc/xdr.h - found
[00:47:54] -- Looking for fopen in c
[00:47:54] -- Looking for fopen in c - found
[00:47:54] -- Looking for __gcc_personality_v0 in gcc_s
[00:47:54] -- Looking for __gcc_personality_v0 in gcc_s - found
[00:47:54] -- Looking for __gcc_personality_v0 in gcc_s - found
[00:47:54] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[00:47:54] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_G_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Success
[00:47:54] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[00:47:54] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[00:47:54] -- Looking for __func__
[00:47:54] -- Looking for __func__ - found
[00:47:54] -- Looking for dlopen in dl - found
[00:47:54] -- Looking for shm_open in rt
[00:47:54] -- Looking for shm_open in rt - found
[00:47:54] -- Looking for pow in m
[00:47:54] -- Looking for pow in m
[00:47:54] -- Looking for pow in m - found
[00:47:54] -- Looking for pthread_create in pthread
[00:47:54] -- Looking for pthread_create in pthread - found
[00:47:54] -- Looking for backtrace in execinfo
[00:47:54] -- Looking for backtrace in execinfo - not found
[00:47:54] -- Looking for __cxa_throw in c++
[00:47:54] -- Looking for __cxa_throw in c++ - not found
[00:47:54] -- Looking for __cxa_throw in stdc++
[00:47:54] -- Looking for __cxa_throw in stdc++ - found
[00:47:54] -- Looking for __i386__
[00:47:54] -- Looking for __i386__ - found
[00:47:54] -- Compiler-RT supported architectures: x86_64;i386
[00:47:54] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[00:47:54] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[00:47:54] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[00:47:54] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[00:47:54] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[00:47:54] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[00:47:54] -- Performing Test HAS_THREAD_LOCAL - Success
[00:47:54] -- Configuring done
[00:47:54] -- Generating done
[00:47:54] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/asan/build
[00:47:54] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/asan/build
[00:47:54] running: "cmake" "--build" "." "--target" "clang_rt.asan-x86_64" "--config" "Release" "--"
[00:47:54] Scanning dependencies of target RTAsan_preinit.x86_64
[00:47:54] [  0%] Building CXX object lib/asan/CMakeFiles/RTAsan_preinit.x86_64.dir/asan_preinit.cc.o
[00:47:54] [  0%] Built target RTAsan_preinit.x86_64
[00:47:54] Scanning dependencies of target RTSanitizerCommonCoverage.x86_64
[00:47:54] [  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sancov_flags.cc.o
[00:47:55] [  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[00:47:55] [  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[00:47:55] [  6%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[00:47:55] [  6%] Built target RTSanitizerCommonCoverage.x86_64
[00:47:55] Scanning dependencies of target RTSanitizerCommon.x86_64
[00:47:55] [  6%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[00:47:55] [  6%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[00:47:55] [ 13%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[00:47:55] [ 13%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[00:47:55] [ 13%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[00:47:55] [ 13%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[00:47:55] [ 13%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[00:47:55] [ 13%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[00:47:55] [ 13%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[00:47:55] [ 13%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[00:47:55] [ 13%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_netbsd.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_openbsd.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_freebsd.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[00:47:55] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_openbsd.cc.o
[00:47:55] [ 26%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[00:47:55] [ 26%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_solaris.cc.o
[00:47:55] [ 26%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_posix.cc.o
[00:47:55] [ 26%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_printf.cc.o
[00:47:55] [ 26%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_common.cc.o
[00:47:55] [ 26%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_bsd.cc.o
[00:47:55] [ 26%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_linux.cc.o
[00:47:55] [ 26%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_mac.cc.o
[00:47:55] [ 26%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_solaris.cc.o
[00:47:55] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_rtems.cc.o
[00:47:55] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_solaris.cc.o
[00:47:55] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_stoptheworld_mac.cc.o
[00:47:55] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_suppressions.cc.o
[00:47:55] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_tls_get_addr.cc.o
[00:47:55] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_thread_registry.cc.o
[00:47:55] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_type_traits.cc.o
[00:47:55] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_win.cc.o
[00:47:55] [ 33%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_x86_64.S.o
[00:47:55] [ 40%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_mips64.S.o
[00:47:55] Scanning dependencies of target RTSanitizerCommonLibc.x86_64
[00:47:55] Scanning dependencies of target RTSanitizerCommonSymbolizer.x86_64
[00:47:55] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cc.o
[00:47:55] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_allocator_report.cc.o
[00:47:55] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_termination.cc.o
[00:47:55] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_allocator_checks.cc.o
[00:47:55] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stackdepot.cc.o
[00:47:55] [ 46%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o
[00:47:55] [ 46%] Built target RTSanitizerCommon.x86_64
[00:47:55] [ 46%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_mac_libcdep.cc.o
[00:47:55] [ 46%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_posix_libcdep.cc.o
[00:47:55] Scanning dependencies of target RTInterception.x86_64
[00:47:55] [ 46%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_linux.cc.o
[00:47:55] [ 46%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace.cc.o
[00:47:55] [ 46%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_mac.cc.o
[00:47:55] [ 46%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_win.cc.o
[00:47:55] [ 46%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_type_test.cc.o
[00:47:55] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace_libcdep.cc.o
[00:47:55] [ 53%] Built target RTInterception.x86_64
[00:47:55] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace_printer.cc.o
[00:47:55] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace_sparc.cc.o
[00:47:55] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer.cc.o
[00:47:55] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_libbacktrace.cc.o
[00:47:55] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_libcdep.cc.o
[00:47:55] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_mac.cc.o
[00:47:55] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_markup.cc.o
[00:47:55] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_posix_libcdep.cc.o
[00:47:55] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_report.cc.o
[00:47:55] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_win.cc.o
[00:47:55] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_unwind_linux_libcdep.cc.o
[00:47:55] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_unwind_win.cc.o
[00:47:55] [ 60%] Built target RTSanitizerCommonSymbolizer.x86_64
[00:47:55] --- stderr
[00:47:55] --- stderr
[00:47:55] /checkout/src/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_linux_libcdep.cc:706:10: error: use of undeclared identifier 'CPU_COUNT'
[00:47:55]   return CPU_COUNT(&CPUs);
[00:47:55] 1 error generated.
[00:47:55] 1 error generated.
[00:47:55] gmake[3]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o] Error 1
[00:47:55] gmake[3]: *** Waiting for unfinished jobs....
[00:47:55] gmake[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/all] Error 2
[00:47:55] gmake[2]: *** Waiting for unfinished jobs....
[00:47:55] gmake[1]: *** [lib/asan/CMakeFiles/clang_rt.asan-x86_64.dir/rule] Error 2
[00:47:55] gmake: *** [clang_rt.asan-x86_64] Error 2
[00:47:55] command did not execute successfully, got: exit code: 2
[00:47:55] 
[00:47:55] 
[00:47:55] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
[00:47:55] 
[00:47:55] warning: build failed, waiting for other jobs to finish...
[00:47:55] [RUSTC-TIMING] core test:false 48.211
[00:47:55] error: build failed
---
travis_time:end:049253e4:start=1558513288549342795,finish=1558513288556641741,duration=7298946
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0778f21a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13cb2bc2
travis_time:start:13cb2bc2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:105bb776
$ dmesg | grep -i kill
