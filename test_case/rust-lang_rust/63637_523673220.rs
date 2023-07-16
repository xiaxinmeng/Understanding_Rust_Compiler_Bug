plain
2019-08-21T22:22:18.5525350Z [RUSTC-TIMING] build_script_build test:false 0.488
2019-08-21T22:22:49.7492590Z error: failed to run custom build command for `rustc_msan v0.0.0 (/Users/vsts/agent/2.155.1/work/1/s/src/librustc_msan)`
2019-08-21T22:22:49.7492940Z 
2019-08-21T22:22:49.7493030Z Caused by:
2019-08-21T22:22:49.7493940Z   process didn't exit successfully: `/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage1-std/release/build/rustc_msan-1ac554e3394dbd25/build-script-build` (exit code: 101)
2019-08-21T22:22:49.7494820Z --- stdout
2019-08-21T22:22:49.7495450Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
2019-08-21T22:22:49.7496260Z cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
2019-08-21T22:22:49.7497100Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/www/menu.html.incl
2019-08-21T22:22:49.7497850Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/www/menu.css
2019-08-21T22:22:49.7498590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/www/index.html
2019-08-21T22:22:49.7499550Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/www/content.css
2019-08-21T22:22:49.7500380Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/utils/generate_netbsd_syscalls.awk
2019-08-21T22:22:49.7501220Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/utils/generate_netbsd_ioctls.awk
2019-08-21T22:22:49.7502190Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/unittests/lit_unittest_cfg_utils.py
2019-08-21T22:22:49.7503020Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/unittests/lit.common.unit.configured.in
2019-08-21T22:22:49.7503810Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/unittests/lit.common.unit.cfg.py
2019-08-21T22:22:49.7504560Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/unittests/CMakeLists.txt
2019-08-21T22:22:49.7505700Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/Unit/lit.site.cfg.py.in
2019-08-21T22:22:49.7506590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/quiet-start.cc
2019-08-21T22:22:49.7507360Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/profiling-single-threaded.cc
2019-08-21T22:22:49.7508200Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/profiling-multi-threaded.cc
2019-08-21T22:22:49.7508990Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/pic_test.cc
2019-08-21T22:22:49.7509750Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/patching-unpatching.cc
2019-08-21T22:22:49.7510560Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/optional-inmemory-log.cc
2019-08-21T22:22:49.7511340Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/logging-modes.cc
2019-08-21T22:22:49.7512120Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/func-id-utils.cc
2019-08-21T22:22:49.7513220Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fork_basic_logging.cc
2019-08-21T22:22:49.7513990Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fixedsize-logging.cc
2019-08-21T22:22:49.7514780Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-thread-order.cc
2019-08-21T22:22:49.7515550Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-single-thread.cc
2019-08-21T22:22:49.7516340Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-reinit.cc
2019-08-21T22:22:49.7517110Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-mode.cc
2019-08-21T22:22:49.7517890Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-mode-multiple.cc
2019-08-21T22:22:49.7518950Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/fdr-mode-inmemory.cc
2019-08-21T22:22:49.7519750Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/custom-event-logging.cc
2019-08-21T22:22:49.7520580Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/custom-event-handler-alignment.cc
2019-08-21T22:22:49.7521400Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/coverage-sample.cc
2019-08-21T22:22:49.7522210Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/common-trampoline-alignment.cc
2019-08-21T22:22:49.7523030Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/clang-no-xray-instrument.cc
2019-08-21T22:22:49.7523830Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/c-test.cc
2019-08-21T22:22:49.7524650Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/basic-filtering.cc
2019-08-21T22:22:49.7525440Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/argv0-log-file-name.cc
2019-08-21T22:22:49.7526480Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/arg1-logging-implicit-this.cc
2019-08-21T22:22:49.7527390Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/arg1-logger.cc
2019-08-21T22:22:49.7528190Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/arg1-arg0-logging.cc
2019-08-21T22:22:49.7529040Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/TestCases/Posix/always-never-instrument.cc
2019-08-21T22:22:49.7529810Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/lit.site.cfg.py.in
2019-08-21T22:22:49.7530590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/lit.cfg.py
2019-08-21T22:22:49.7531390Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/xray/CMakeLists.txt
2019-08-21T22:22:49.7532190Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/uadd-overflow.cpp
2019-08-21T22:22:49.7533000Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/test-darwin-interface.c
2019-08-21T22:22:49.7534100Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/recover-dedup.cpp
2019-08-21T22:22:49.7534930Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/recover-dedup-limit.cpp
2019-08-21T22:22:49.7535740Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-unsigned-integer-truncation.c
2019-08-21T22:22:49.7536600Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-signed-integer-truncation.c
2019-08-21T22:22:49.7537590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-signed-integer-truncation-or-sign-change.c
2019-08-21T22:22:49.7538550Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/implicit-integer-sign-change.c
2019-08-21T22:22:49.7539420Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/TestCases/alignment-assumption.c
2019-08-21T22:22:49.7540200Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/lit.site.cfg.py.in
2019-08-21T22:22:49.7541000Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/lit.common.cfg.py
2019-08-21T22:22:49.7542630Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan_minimal/CMakeLists.txt
2019-08-21T22:22:49.7543440Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr.cpp
2019-08-21T22:22:49.7544280Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr-virtual-base.cpp
2019-08-21T22:22:49.7545090Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr-virtual-base-construction.cpp
2019-08-21T22:22:49.7545950Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr-non-unique-typeinfo.cpp
2019-08-21T22:22:49.7546780Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/vptr-corrupted-vtable-itanium.cpp
2019-08-21T22:22:49.7547590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/PR33221.cpp
2019-08-21T22:22:49.7548610Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/null.cpp
2019-08-21T22:22:49.7549500Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/misaligned.cpp
2019-08-21T22:22:49.7550320Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/Linux/PR33221.cpp
2019-08-21T22:22:49.7551160Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/Linux/lit.local.cfg.py
2019-08-21T22:22:49.7551960Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/Function/lit.local.cfg.py
2019-08-21T22:22:49.7552780Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/TypeCheck/Function/function.cpp
2019-08-21T22:22:49.7553600Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/unsigned-index-expression.cpp
2019-08-21T22:22:49.7554420Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/index-overflow.cpp
2019-08-21T22:22:49.7555210Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-summary.cpp
2019-08-21T22:22:49.7556350Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-openmp.cpp
2019-08-21T22:22:49.7557220Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-builtin_assume_aligned-two-params.cpp
2019-08-21T22:22:49.7558090Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-builtin_assume_aligned-three-params.cpp
2019-08-21T22:22:49.7558990Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-builtin_assume_aligned-three-params-variable.cpp
2019-08-21T22:22:49.7559850Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-blacklist.cpp
2019-08-21T22:22:49.7560720Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-assume_aligned-on-function.cpp
2019-08-21T22:22:49.7561610Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-assume_aligned-on-function-two-params.cpp
2019-08-21T22:22:49.7562500Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-alloc_align-on-function.cpp
2019-08-21T22:22:49.7563370Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-alloc_align-on-function-variable.cpp
2019-08-21T22:22:49.7564420Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-align_value-on-paramvar.cpp
2019-08-21T22:22:49.7565420Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Pointer/alignment-assumption-attribute-align_value-on-lvalue.cpp
2019-08-21T22:22:49.7566240Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/vla.c
2019-08-21T22:22:49.7567050Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/unreachable.cpp
2019-08-21T22:22:49.7568030Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/nullability.c
2019-08-21T22:22:49.7568940Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/nonnull.cpp
2019-08-21T22:22:49.7569740Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/nonnull-arg.cpp
2019-08-21T22:22:49.7570560Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/no-interception.cpp
2019-08-21T22:22:49.7571370Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/monitor.cpp
2019-08-21T22:22:49.7572150Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/missing_return.cpp
2019-08-21T22:22:49.7572970Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/log-path_test.cc
2019-08-21T22:22:49.7573780Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Linux/ubsan_options.cc
2019-08-21T22:22:49.7574580Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Linux/print_stack_trace.cc
2019-08-21T22:22:49.7575700Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Linux/lit.local.cfg.py
2019-08-21T22:22:49.7576510Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Inputs/returns-unexpectedly.c
2019-08-21T22:22:49.7577340Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/Inputs/no-interception-dso.c
2019-08-21T22:22:49.7578110Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/enum.cpp
2019-08-21T22:22:49.7578940Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/deduplication.cpp
2019-08-21T22:22:49.7579750Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/coverage-levels.cc
2019-08-21T22:22:49.7580540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/builtins.cpp
2019-08-21T22:22:49.7581350Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/bounds.cpp
2019-08-21T22:22:49.7582120Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/bool.m
2019-08-21T22:22:49.7582920Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Misc/bool.cpp
2019-08-21T22:22:49.7583720Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/usub-overflow.cpp
2019-08-21T22:22:49.7584540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/umul-overflow.cpp
2019-08-21T22:22:49.7585360Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/uincdec-overflow.cpp
2019-08-21T22:22:49.7586180Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/uadd-overflow.cpp
2019-08-21T22:22:49.7586990Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/suppressions.cpp
2019-08-21T22:22:49.7587770Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/summary.cpp
2019-08-21T22:22:49.7588900Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/sub-overflow.cpp
2019-08-21T22:22:49.7589800Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/shift.cpp
2019-08-21T22:22:49.7590760Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/no-recover.cpp
2019-08-21T22:22:49.7591580Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/negate-overflow.cpp
2019-08-21T22:22:49.7592390Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/mul-overflow.cpp
2019-08-21T22:22:49.7593210Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/incdec-overflow.cpp
2019-08-21T22:22:49.7593980Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/div-zero.cpp
2019-08-21T22:22:49.7594810Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/div-overflow.cpp
2019-08-21T22:22:49.7595730Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Integer/add-overflow.cpp
2019-08-21T22:22:49.7596550Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/unsigned-integer-truncation.c
2019-08-21T22:22:49.7597780Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/unsigned-integer-truncation-summary.cpp
2019-08-21T22:22:49.7598660Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/unsigned-integer-truncation-blacklist.c
2019-08-21T22:22:49.7599530Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation.c
2019-08-21T22:22:49.7600380Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation-summary.cpp
2019-08-21T22:22:49.7601240Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation-or-sign-change-summary.cpp
2019-08-21T22:22:49.7602160Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation-or-sign-change-blacklist.c
2019-08-21T22:22:49.7603020Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/signed-integer-truncation-blacklist.c
2019-08-21T22:22:49.7603850Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-truncation.c
2019-08-21T22:22:49.7604700Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-sign-change.c
2019-08-21T22:22:49.7605530Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-sign-change-summary.cpp
2019-08-21T22:22:49.7606520Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-sign-change-blacklist.c
2019-08-21T22:22:49.7607420Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-conversion.c
2019-08-21T22:22:49.7608360Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/ImplicitConversion/integer-arithmetic-value-change.c
2019-08-21T22:22:49.7609480Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/TestCases/Float/cast-overflow.cpp
2019-08-21T22:22:49.7610370Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/lit.site.cfg.py.in
2019-08-21T22:22:49.7611170Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/lit.common.cfg.py
2019-08-21T22:22:49.7611950Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/ubsan/CMakeLists.txt
2019-08-21T22:22:49.7612790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/write_in_reader_lock.cc
2019-08-21T22:22:49.7613580Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/vptr_harmful_race4.cc
2019-08-21T22:22:49.7614340Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/vptr_harmful_race3.cc
2019-08-21T22:22:49.7615140Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/vptr_harmful_race2.cc
2019-08-21T22:22:49.7615910Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/vptr_harmful_race.cc
2019-08-21T22:22:49.7616690Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/vptr_benign_race.cc
2019-08-21T22:22:49.7617460Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/virtual_inheritance_compile_bug.cc
2019-08-21T22:22:49.7618540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/vfork.cc
2019-08-21T22:22:49.7619320Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Unit/lit.site.cfg.py.in
2019-08-21T22:22:49.7620090Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/unaligned_race.cc
2019-08-21T22:22:49.7620890Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/unaligned_norace.cc
2019-08-21T22:22:49.7621660Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/tsan-vs-gvn.cc
2019-08-21T22:22:49.7622440Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/tls_race2.cc
2019-08-21T22:22:49.7623190Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/tls_race.cc
2019-08-21T22:22:49.7623990Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/tiny_race.c
2019-08-21T22:22:49.7624760Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_name2.cc
2019-08-21T22:22:49.7625520Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_name.cc
2019-08-21T22:22:49.7626290Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_leak5.c
2019-08-21T22:22:49.7627060Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_leak4.c
2019-08-21T22:22:49.7627850Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_leak3.c
2019-08-21T22:22:49.7628710Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_leak2.c
2019-08-21T22:22:49.7629520Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_leak.c
2019-08-21T22:22:49.7630290Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_exit.c
2019-08-21T22:22:49.7631070Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_end_with_ignore3.cc
2019-08-21T22:22:49.7631860Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_end_with_ignore2.cc
2019-08-21T22:22:49.7632860Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_end_with_ignore.cc
2019-08-21T22:22:49.7633750Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_detach2.c
2019-08-21T22:22:49.7634510Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/thread_detach.c
2019-08-21T22:22:49.7635300Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/test.h
2019-08-21T22:22:49.7636090Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppressions_race2.cc.supp
2019-08-21T22:22:49.7636860Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppressions_race2.cc
2019-08-21T22:22:49.7637640Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppressions_race.cc.supp
2019-08-21T22:22:49.7638540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppressions_race.cc
2019-08-21T22:22:49.7639350Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppressions_mutex.cc.supp
2019-08-21T22:22:49.7640120Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppressions_mutex.cc
2019-08-21T22:22:49.7641210Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppressions_global.cc.supp
2019-08-21T22:22:49.7642010Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppressions_global.cc
2019-08-21T22:22:49.7642790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppress_same_stacks.cc
2019-08-21T22:22:49.7643600Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/suppress_same_address.cc
2019-08-21T22:22:49.7644360Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/sunrpc.cc
2019-08-21T22:22:49.7645140Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/strerror_r.cc
2019-08-21T22:22:49.7645890Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/static_init6.cc
2019-08-21T22:22:49.7646690Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/static_init5.cc
2019-08-21T22:22:49.7647440Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/static_init4.cc
2019-08-21T22:22:49.7648220Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/static_init3.cc
2019-08-21T22:22:49.7648990Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/static_init2.cc
2019-08-21T22:22:49.7649770Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/static_init1.cc
2019-08-21T22:22:49.7650550Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/stack_sync_reuse.cc
2019-08-21T22:22:49.7651310Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/stack_race2.cc
2019-08-21T22:22:49.7652100Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/stack_race.cc
2019-08-21T22:22:49.7652860Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/sleep_sync2.cc
2019-08-21T22:22:49.7653640Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/sleep_sync.cc
2019-08-21T22:22:49.7654420Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/simple_stack2.cc
2019-08-21T22:22:49.7655390Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/simple_stack.c
2019-08-21T22:22:49.7656260Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/simple_race.cc
2019-08-21T22:22:49.7657020Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/simple_race.c
2019-08-21T22:22:49.7657830Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/sigsuspend.cc
2019-08-21T22:22:49.7658580Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_write.cc
2019-08-21T22:22:49.7659360Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_thread.cc
2019-08-21T22:22:49.7660220Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_sync2.cc
2019-08-21T22:22:49.7661040Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_sync.cc
2019-08-21T22:22:49.7661820Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_reset.cc
2019-08-21T22:22:49.7662590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_recursive.cc
2019-08-21T22:22:49.7663380Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_pause.cc
2019-08-21T22:22:49.7664430Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_malloc.cc
2019-08-21T22:22:49.7665230Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_longjmp.cc
2019-08-21T22:22:49.7665980Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_errno.cc
2019-08-21T22:22:49.7666780Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_cond.cc
2019-08-21T22:22:49.7667560Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/signal_block.cc
2019-08-21T22:22:49.7668310Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/setuid2.c
2019-08-21T22:22:49.7669080Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/setuid.c
2019-08-21T22:22:49.7669850Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/restore_stack.cc
2019-08-21T22:22:49.7670670Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/real_deadlock_detector_stress_test.cc
2019-08-21T22:22:49.7671440Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_with_finished_thread.cc
2019-08-21T22:22:49.7672250Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_top_suppression1.cc
2019-08-21T22:22:49.7673040Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_top_suppression.cc
2019-08-21T22:22:49.7673790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_stress.cc
2019-08-21T22:22:49.7674560Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_write.cc
2019-08-21T22:22:49.7675350Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_speculative_load.cc
2019-08-21T22:22:49.7676140Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_read.cc
2019-08-21T22:22:49.7676890Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_puts.cc
2019-08-21T22:22:49.7677870Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_mutex2.c
2019-08-21T22:22:49.7678750Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_mutex.c
2019-08-21T22:22:49.7679520Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_heap.cc
2019-08-21T22:22:49.7680300Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_fputs.cc
2019-08-21T22:22:49.7681080Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_barrier2.c
2019-08-21T22:22:49.7681860Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/race_on_barrier.c
2019-08-21T22:22:49.7682620Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/pthread_key.cc
2019-08-21T22:22:49.7683420Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/pthread_atfork_deadlock.c
2019-08-21T22:22:49.7684200Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/printf-1.c
2019-08-21T22:22:49.7684950Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/pie_test.cc
2019-08-21T22:22:49.7685760Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutexset8.cc
2019-08-21T22:22:49.7686810Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutexset7.cc
2019-08-21T22:22:49.7687590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutexset6.cc
2019-08-21T22:22:49.7688430Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutexset5.cc
2019-08-21T22:22:49.7689230Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutexset4.cc
2019-08-21T22:22:49.7689990Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutexset3.cc
2019-08-21T22:22:49.7690920Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutexset2.cc
2019-08-21T22:22:49.7691700Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutexset1.cc
2019-08-21T22:22:49.7692500Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_lock_destroyed.cc
2019-08-21T22:22:49.7693270Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_double_lock.cc
2019-08-21T22:22:49.7694040Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_destroy_locked2.cc
2019-08-21T22:22:49.7694830Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_destroy_locked.cc
2019-08-21T22:22:49.7695600Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_cycle_long.c
2019-08-21T22:22:49.7696380Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_cycle2.c
2019-08-21T22:22:49.7697160Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_bad_unlock.cc
2019-08-21T22:22:49.7697950Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_bad_read_unlock.cc
2019-08-21T22:22:49.7698730Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_bad_read_lock.cc
2019-08-21T22:22:49.7699510Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mutex_annotations.cc
2019-08-21T22:22:49.7700290Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/must_deadlock.cc
2019-08-21T22:22:49.7701250Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mop_with_offset2.cc
2019-08-21T22:22:49.7702150Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mop_with_offset.cc
2019-08-21T22:22:49.7702910Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mop1.c
2019-08-21T22:22:49.7703690Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mmap_stress.cc
2019-08-21T22:22:49.7704460Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/mmap_large.cc
2019-08-21T22:22:49.7705210Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/memcpy_race.cc
2019-08-21T22:22:49.7706000Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/memcmp_race.cc
2019-08-21T22:22:49.7706760Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/map32bit.cc
2019-08-21T22:22:49.7707540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/malloc_stack.cc
2019-08-21T22:22:49.7708300Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/malloc_overflow.cc
2019-08-21T22:22:49.7709090Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/lots_of_threads.c
2019-08-21T22:22:49.7710160Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/longjmp4.cc
2019-08-21T22:22:49.7710920Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/longjmp3.cc
2019-08-21T22:22:49.7711700Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/longjmp2.cc
2019-08-21T22:22:49.7712450Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/longjmp.cc
2019-08-21T22:22:49.7713240Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/load_shared_lib.cc
2019-08-21T22:22:49.7714010Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/lit.site.cfg.py.in
2019-08-21T22:22:49.7714770Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/lit.cfg.py
2019-08-21T22:22:49.7715560Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/user_malloc.cc
2019-08-21T22:22:49.7716330Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/user_fopen.cc
2019-08-21T22:22:49.7717120Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/thread_tryjoin.c
2019-08-21T22:22:49.7717890Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/thread_timedjoin.c
2019-08-21T22:22:49.7718790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/pie_no_aslr.cc
2019-08-21T22:22:49.7719570Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/mutex_robust2.cc
2019-08-21T22:22:49.7720360Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/mutex_robust.cc
2019-08-21T22:22:49.7721160Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/lit.local.cfg.py
2019-08-21T22:22:49.7721930Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/double_race.cc
2019-08-21T22:22:49.7722710Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/check_preinit.cc
2019-08-21T22:22:49.7723690Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Linux/check_memcpy.c
2019-08-21T22:22:49.7724600Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/target-queue-norace.c
2019-08-21T22:22:49.7725370Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/sync-race.c
2019-08-21T22:22:49.7726170Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/sync-norace.c
2019-08-21T22:22:49.7726980Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/sync-block-copy.cc
2019-08-21T22:22:49.7727750Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/suspend.c
2019-08-21T22:22:49.7728540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/source-serial.c
2019-08-21T22:22:49.7729340Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/source-registration2.c
2019-08-21T22:22:49.7730150Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/source-registration.c
2019-08-21T22:22:49.7730920Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/source-event2.c
2019-08-21T22:22:49.7732010Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/source-event.c
2019-08-21T22:22:49.7732800Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/source-cancel2.c
2019-08-21T22:22:49.7733590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/source-cancel.c
2019-08-21T22:22:49.7734390Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/serial-queue-norace.c
2019-08-21T22:22:49.7735200Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/semaphore-norace.c
2019-08-21T22:22:49.7736000Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/once.c
2019-08-21T22:22:49.7736760Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/lit.local.cfg.py
2019-08-21T22:22:49.7737570Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/io.c
2019-08-21T22:22:49.7738440Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/io-race.c
2019-08-21T22:22:49.7739240Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/io-cleanup.c
2019-08-21T22:22:49.7740030Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/io-barrier.c
2019-08-21T22:22:49.7740820Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/io-barrier-race.c
2019-08-21T22:22:49.7741630Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/groups-stress.c
2019-08-21T22:22:49.7742400Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/groups-norace.c
2019-08-21T22:22:49.7743210Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/groups-leave.c
2019-08-21T22:22:49.7744010Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/groups-destructor.cc
2019-08-21T22:22:49.7744780Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/fd.c
2019-08-21T22:22:49.7745790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/dispatch_once_deadlock.c
2019-08-21T22:22:49.7746660Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/dispatch_main.c
2019-08-21T22:22:49.7747460Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/data.c
2019-08-21T22:22:49.7748240Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/blocks.c
2019-08-21T22:22:49.7749030Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/barrier.c
2019-08-21T22:22:49.7749810Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/barrier-race.c
2019-08-21T22:22:49.7750590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/async-race.c
2019-08-21T22:22:49.7751400Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/async-norace.c
2019-08-21T22:22:49.7752180Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/apply.c
2019-08-21T22:22:49.7752970Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/apply-race.c
2019-08-21T22:22:49.7754180Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libdispatch/after.c
2019-08-21T22:22:49.7754980Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libcxx/std_shared_ptr.cc
2019-08-21T22:22:49.7755770Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/libcxx/lit.local.cfg.py
2019-08-21T22:22:49.7756540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/large_malloc_meta.cc
2019-08-21T22:22:49.7757340Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_volatile.cc
2019-08-21T22:22:49.7758130Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_symbolization_legacy.cc
2019-08-21T22:22:49.7758920Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_symbolization.cc
2019-08-21T22:22:49.7759680Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_rwlock.cc
2019-08-21T22:22:49.7760460Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_race_pc.cc
2019-08-21T22:22:49.7761230Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_race_move.cc
2019-08-21T22:22:49.7762120Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_race.cc
2019-08-21T22:22:49.7762900Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_move_overlap_race.cc
2019-08-21T22:22:49.7763650Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_move_overlap.cc
2019-08-21T22:22:49.7764420Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_lock_rec_race.cc
2019-08-21T22:22:49.7765170Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_lock_rec.cc
2019-08-21T22:22:49.7765920Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_lock_move.cc
2019-08-21T22:22:49.7766670Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_lock.cc
2019-08-21T22:22:49.7767400Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_heap_init.cc
2019-08-21T22:22:49.7768410Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_find.cc
2019-08-21T22:22:49.7769250Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_finalizer.cc
2019-08-21T22:22:49.7770010Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java_alloc.cc
2019-08-21T22:22:49.7770750Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/java.h
2019-08-21T22:22:49.7771510Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/interface_atomic_test.c
2019-08-21T22:22:49.7772240Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/inlined_memcpy_race2.cc
2019-08-21T22:22:49.7773010Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/inlined_memcpy_race.cc
2019-08-21T22:22:49.7773790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignored-interceptors-mmap.cc
2019-08-21T22:22:49.7774540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_sync.cc
2019-08-21T22:22:49.7775290Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_race.cc
2019-08-21T22:22:49.7776310Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_malloc.cc
2019-08-21T22:22:49.7777070Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib_lib.h
2019-08-21T22:22:49.7777950Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib5.cc.supp
2019-08-21T22:22:49.7778740Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib5.cc
2019-08-21T22:22:49.7779530Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib4.cc
2019-08-21T22:22:49.7780300Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib3.cc.supp
2019-08-21T22:22:49.7781230Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib3.cc
2019-08-21T22:22:49.7781970Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib2.cc.supp
2019-08-21T22:22:49.7782750Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib2.cc
2019-08-21T22:22:49.7783480Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib1.cc.supp
2019-08-21T22:22:49.7784230Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib1.cc
2019-08-21T22:22:49.7785110Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib0.cc.supp
2019-08-21T22:22:49.7785900Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_lib0.cc
2019-08-21T22:22:49.7786660Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/ignore_free.cc
2019-08-21T22:22:49.7787380Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/heap_race.cc
2019-08-21T22:22:49.7788460Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/halt_on_error.cc
2019-08-21T22:22:49.7789240Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/global_race3.cc
2019-08-21T22:22:49.7790030Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/global_race2.cc
2019-08-21T22:22:49.7791490Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/global_race.cc
2019-08-21T22:22:49.7803860Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/getline_nohang.cc
2019-08-21T22:22:49.7804820Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/free_race2.c
2019-08-21T22:22:49.7805600Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/free_race.c.supp
2019-08-21T22:22:49.7806390Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/free_race.c
2019-08-21T22:22:49.7807160Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fork_multithreaded3.cc
2019-08-21T22:22:49.7807960Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fork_multithreaded.cc
2019-08-21T22:22:49.7808830Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fork_deadlock.cc
2019-08-21T22:22:49.7809640Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fork_atexit.cc
2019-08-21T22:22:49.7810430Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fiber_two_threads.cc
2019-08-21T22:22:49.7811190Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fiber_simple.cc
2019-08-21T22:22:49.7812280Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fiber_race.cc
2019-08-21T22:22:49.7813060Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fiber_longjmp.cc
2019-08-21T22:22:49.7813850Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fiber_from_thread.cc
2019-08-21T22:22:49.7814610Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fiber_asm.cc
2019-08-21T22:22:49.7815420Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_tid_recycled.cc
2019-08-21T22:22:49.7816200Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_stdout_race.cc
2019-08-21T22:22:49.7816980Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_socketpair_norace.cc
2019-08-21T22:22:49.7817790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_socket_norace.cc
2019-08-21T22:22:49.7818570Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_socket_connect_norace.cc
2019-08-21T22:22:49.7819360Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_pipe_race.cc
2019-08-21T22:22:49.7820120Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_pipe_norace.cc
2019-08-21T22:22:49.7820910Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_location.cc
2019-08-21T22:22:49.7821690Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_dup_race.cc
2019-08-21T22:22:49.7822450Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_dup_norace2.cc
2019-08-21T22:22:49.7823250Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_dup_norace.cc
2019-08-21T22:22:49.7824020Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_close_norace2.cc
2019-08-21T22:22:49.7824810Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/fd_close_norace.cc
2019-08-21T22:22:49.7825560Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/exceptions.cc
2019-08-21T22:22:49.7826540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/dtls.c
2019-08-21T22:22:49.7827390Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/dlclose.cc
2019-08-21T22:22:49.7828320Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/dl_iterate_phdr.cc
2019-08-21T22:22:49.7829200Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/deflake.bash
2019-08-21T22:22:49.7830030Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/default_options.cc
2019-08-21T22:22:49.7830880Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/deep_stack1.cc
2019-08-21T22:22:49.7831670Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/debugging.cc
2019-08-21T22:22:49.7832470Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/debug_locate.cc
2019-08-21T22:22:49.7833230Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/debug_alloc_stack.cc
2019-08-21T22:22:49.7834030Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/deadlock_detector_stress_test.cc
2019-08-21T22:22:49.7834810Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/xpc.mm
2019-08-21T22:22:49.7835870Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/xpc-race.mm
2019-08-21T22:22:49.7836670Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/xpc-cancel.mm
2019-08-21T22:22:49.7837440Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/workerthreads.mm
2019-08-21T22:22:49.7838350Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/symbolizer-dladdr.cc
2019-08-21T22:22:49.7839160Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/symbolizer-atos.cc
2019-08-21T22:22:49.7839960Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/signals-blocked.cc
2019-08-21T22:22:49.7840770Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/realloc-zero.cc
2019-08-21T22:22:49.7841550Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/osspinlock-norace.cc
2019-08-21T22:22:49.7842350Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/osatomics-list.mm
2019-08-21T22:22:49.7843130Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/osatomics-bitops.mm
2019-08-21T22:22:49.7843940Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/osatomics-add.mm
2019-08-21T22:22:49.7844720Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/objc-synchronize.mm
2019-08-21T22:22:49.7845540Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/objc-synchronize-tagged.mm
2019-08-21T22:22:49.7846380Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/objc-synchronize-nested-recursive.mm
2019-08-21T22:22:49.7847180Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/objc-synchronize-cycle.mm
2019-08-21T22:22:49.7848000Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/objc-synchronize-cycle-tagged.mm
2019-08-21T22:22:49.7848980Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/objc-simple.mm
2019-08-21T22:22:49.7849880Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/objc-race.mm
2019-08-21T22:22:49.7850660Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/objc-double-property.mm
2019-08-21T22:22:49.7851480Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/norace-objcxx-run-time.mm
2019-08-21T22:22:49.7852840Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/malloc_size.mm
2019-08-21T22:22:49.7853740Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/malloc-stack-logging.cc
2019-08-21T22:22:49.7854550Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/main_tid.mm
2019-08-21T22:22:49.7855350Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/lit.local.cfg.py
2019-08-21T22:22:49.7856160Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/libcxx-shared-ptr.mm
2019-08-21T22:22:49.7856940Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/libcxx-shared-ptr-stress.mm
2019-08-21T22:22:49.7858330Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/libcxx-shared-ptr-recursive.mm
2019-08-21T22:22:49.7859140Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/libcxx-future.mm
2019-08-21T22:22:49.7859920Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/libcxx-call-once.mm
2019-08-21T22:22:49.7860960Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/ignore-noninstrumented.mm
2019-08-21T22:22:49.7861930Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/gcd-sync-block-copy.mm
2019-08-21T22:22:49.7862700Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/external.cc
2019-08-21T22:22:49.7863450Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/external-swift.cc
2019-08-21T22:22:49.7864250Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/external-swift-debugging.cc
2019-08-21T22:22:49.7865040Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/external-noninstrumented-module.cc
2019-08-21T22:22:49.7865790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/external-lib.cc
2019-08-21T22:22:49.7866810Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/external-ignore-noninstrumented.cc
2019-08-21T22:22:49.7867580Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/external-dups.cc
2019-08-21T22:22:49.7868340Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/dlopen.cc
2019-08-21T22:22:49.7869250Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/debug_external.cc
2019-08-21T22:22:49.7870050Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/Darwin/deadlock.mm
2019-08-21T22:22:49.7870830Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/cxa_guard_acquire.cc
2019-08-21T22:22:49.7871600Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/custom_mutex5.cc
2019-08-21T22:22:49.7872720Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/custom_mutex4.cc
2019-08-21T22:22:49.7873550Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/custom_mutex3.cc
2019-08-21T22:22:49.7874330Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/custom_mutex2.cc
2019-08-21T22:22:49.7875060Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/custom_mutex1.cc
2019-08-21T22:22:49.7875840Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/custom_mutex0.cc
2019-08-21T22:22:49.7876590Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/custom_mutex.h
2019-08-21T22:22:49.7877320Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/cond_version.c
2019-08-21T22:22:49.7878220Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/cond_race.cc
2019-08-21T22:22:49.7879000Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/cond_destruction.cc
2019-08-21T22:22:49.7879790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/cond_cancel.c
2019-08-21T22:22:49.7880530Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/cond.c
2019-08-21T22:22:49.7881720Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/CMakeLists.txt
2019-08-21T22:22:49.7882480Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/blacklist2.cc
2019-08-21T22:22:49.7883210Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/blacklist.cc
2019-08-21T22:22:49.7883950Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/benign_race.cc
2019-08-21T22:22:49.7884710Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench_ten_mutexes.cc
2019-08-21T22:22:49.7885490Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench_single_writer.cc
2019-08-21T22:22:49.7886230Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench_shadow_flush.cc
2019-08-21T22:22:49.7887000Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench_rwmutex.cc
2019-08-21T22:22:49.7887740Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench_release_only.cc
2019-08-21T22:22:49.7888610Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench_mutex.cc
2019-08-21T22:22:49.7889370Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench_local_mutex.cc
2019-08-21T22:22:49.7890130Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench_acquire_release.cc
2019-08-21T22:22:49.7890900Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench_acquire_only.cc
2019-08-21T22:22:49.7891630Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/bench.h
2019-08-21T22:22:49.7892390Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/barrier.cc
2019-08-21T22:22:49.7893130Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atomic_store.cc
2019-08-21T22:22:49.7893880Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atomic_stack.cc
2019-08-21T22:22:49.7894620Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atomic_race.cc
2019-08-21T22:22:49.7895560Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atomic_norace.cc
2019-08-21T22:22:49.7896410Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atomic_hle.cc
2019-08-21T22:22:49.7897160Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atomic_free3.cc
2019-08-21T22:22:49.7898080Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atomic_free2.cc
2019-08-21T22:22:49.7898840Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atomic_free.cc
2019-08-21T22:22:49.7899610Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atexit3.cc
2019-08-21T22:22:49.7900370Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atexit2.cc
2019-08-21T22:22:49.7901280Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/atexit.cc
2019-08-21T22:22:49.7902040Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/annotate_happens_before.cc
2019-08-21T22:22:49.7902790Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/tsan/aligned_vs_unaligned_race.cc
2019-08-21T22:22:49.7903560Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/shadowcallstack/overflow.c
2019-08-21T22:22:49.7904580Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/shadowcallstack/minimal_runtime.h
2019-08-21T22:22:49.7905370Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/shadowcallstack/lit.site.cfg.py.in
2019-08-21T22:22:49.7906120Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/shadowcallstack/lit.cfg.py
2019-08-21T22:22:49.7906890Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/shadowcallstack/libc_support.h
2019-08-21T22:22:49.7907650Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/shadowcallstack/init.c
2019-08-21T22:22:49.7908400Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/shadowcallstack/CMakeLists.txt
2019-08-21T22:22:49.7909170Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/scudo/valloc.c
2019-08-21T22:22:49.7909900Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/scudo/tsd_destruction.c
2019-08-21T22:22:49.7910650Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/scudo/threads.c
2019-08-21T22:22:49.7911370Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/scudo/symbols.test
2019-08-21T22:22:49.7912140Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/scudo/stats.c
2019-08-21T22:22:49.7912910Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/scudo/standalone/unit/lit.site.cfg.py.in
2019-08-21T22:22:49.7913670Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/scudo/standalone/CMakeLists.txt
2019-08-21T22:22:49.7914440Z cargo:rerun-if-changed=/Users/vsts/agent/2.155.1/work/1/s/src/llvm-project/compiler-rt/test/scudo/sizes.cpp
---
2019-08-21T22:22:50.0572230Z -- Looking for __cxa_throw in stdc++
2019-08-21T22:22:50.0572850Z -- Looking for __cxa_throw in stdc++ - found
2019-08-21T22:22:50.0573490Z -- Performing Test COMPILER_RT_HAS_Z_TEXT
2019-08-21T22:22:50.0574120Z -- Performing Test COMPILER_RT_HAS_Z_TEXT - Failed
2019-08-21T22:22:50.0574780Z -- Performing Test COMPILER_RT_HAS_APP_EXTENSION
2019-08-21T22:22:50.0575420Z -- Performing Test COMPILER_RT_HAS_APP_EXTENSION - Success
2019-08-21T22:22:50.0576160Z -- Got ld supported ARCHES: armv6 armv7 armv7s arm64 i386 x86_64 x86_64h armv6m armv7k armv7m armv7em (tvOS)
2019-08-21T22:22:50.0576940Z -- Toolchain supported arches: armv6;armv7;armv7s;arm64;i386;x86_64;x86_64h;armv6m;armv7k;armv7m;armv7em;(tvOS)
2019-08-21T22:22:50.0577590Z -- Finding valid architectures for osx...
2019-08-21T22:22:50.0578550Z -- OSX supported arches: i386;x86_64
2019-08-21T22:22:50.0579210Z -- Finding valid architectures for iossim...
2019-08-21T22:22:50.0579890Z -- ios Simulator supported arches: i386;x86_64
2019-08-21T22:22:50.0580560Z -- Finding valid architectures for ios...
2019-08-21T22:22:50.0581390Z -- ios supported arches: armv7;armv7s;arm64;armv7k
2019-08-21T22:22:50.0582070Z -- Compiler-RT supported architectures: i386;x86_64;armv7;armv7s;arm64;armv7k
2019-08-21T22:22:50.0583380Z -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
2019-08-21T22:22:50.0584030Z -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
2019-08-21T22:22:50.0584680Z -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
2019-08-21T22:22:50.0585350Z -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
2019-08-21T22:22:50.0585350Z -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
2019-08-21T22:22:50.0585990Z -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
2019-08-21T22:22:50.0586630Z -- Performing Test HAS_THREAD_LOCAL
2019-08-21T22:22:50.0587250Z -- Performing Test HAS_THREAD_LOCAL - Success
2019-08-21T22:22:50.0587870Z -- Configuring done
2019-08-21T22:22:50.0588460Z -- Generating done
2019-08-21T22:22:50.0589370Z -- Build files have been written to: /Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/native/msan/build
2019-08-21T22:22:50.0590090Z running: "cmake" "--build" "." "--target" "clang_rt.msan_osx_dynamic" "--config" "Release" "--"
2019-08-21T22:22:50.0590790Z --- stderr
2019-08-21T22:22:50.0590920Z CMake Warning at cmake/Modules/CompilerRTUtils.cmake:263 (message):
2019-08-21T22:22:50.0591580Z   llvm-config finding testingsupport failed with status 1
2019-08-21T22:22:50.0591710Z Call Stack (most recent call first):
2019-08-21T22:22:50.0591710Z Call Stack (most recent call first):
2019-08-21T22:22:50.0591780Z   CMakeLists.txt:75 (load_llvm_config)
2019-08-21T22:22:50.0592300Z 
2019-08-21T22:22:50.0592300Z 
2019-08-21T22:22:50.0592980Z make: *** No rule to make target `clang_rt.msan_osx_dynamic'.  Stop.
2019-08-21T22:22:50.0593720Z command did not execute successfully, got: exit code: 2
2019-08-21T22:22:50.0593800Z 
2019-08-21T22:22:50.0593800Z 
2019-08-21T22:22:50.0594510Z build script failed, must exit now', /Users/vsts/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
2019-08-21T22:22:50.0594740Z 
2019-08-21T22:22:50.0594780Z 
2019-08-21T22:22:50.0594830Z 
2019-08-21T22:22:50.0594830Z 
2019-08-21T22:22:50.0595920Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage0/bin/cargo" "test" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/Users/vsts/agent/2.155.1/work/1/s/src/libtest/Cargo.toml" "-p" "rustc_msan" "--"
2019-08-21T22:22:50.0596220Z 
2019-08-21T22:22:50.0596280Z 
2019-08-21T22:22:50.0596360Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-08-21T22:22:50.0596460Z Build completed unsuccessfully in 1:25:55
2019-08-21T22:22:50.0596460Z Build completed unsuccessfully in 1:25:55
2019-08-21T22:22:50.0596790Z == clock drift check ==
2019-08-21T22:22:50.0596880Z   local time: Wed Aug 21 22:22:49 UTC 2019
2019-08-21T22:22:50.0596960Z   network time: Wed, 21 Aug 2019 22:22:49 GMT
2019-08-21T22:22:50.0597050Z == end clock drift check ==
2019-08-21T22:22:50.0681760Z ##[error]Bash exited with code '1'.
2019-08-21T22:22:50.0757310Z ##[section]Starting: Upload CPU usage statistics
2019-08-21T22:22:50.0761880Z ==============================================================================
2019-08-21T22:22:50.0761980Z Task         : Bash
2019-08-21T22:22:50.0762070Z Description  : Run a Bash script on macOS, Linux, or Windows
