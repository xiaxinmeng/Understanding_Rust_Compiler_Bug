plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
# Compile the test program with non-experimental coverage instrumentation, and generate LLVM IR
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-llvmir/coverage-llvmir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-llvmir/coverage-llvmir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-llvmir/coverage-llvmir  ../coverage-llvmir/testprog.rs \
  -Cinstrument-coverage \
  --emit=llvm-ir
cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-llvmir/coverage-llvmir"/testprog.ll | \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" ../coverage-llvmir/filecheck.testprog.txt -check-prefixes=CHECK -DDEFINE_INTERNAL='define internal' -DCOMDAT_IF_SUPPORTED=', comdat' -DINSTR_PROF_DATA='__llvm_prf_data' -DINSTR_PROF_NAME='__llvm_prf_names' -DINSTR_PROF_CNTS='__llvm_prf_cnts' -DINSTR_PROF_VALS='__llvm_prf_vals' -DINSTR_PROF_VNODES='__llvm_prf_vnds' -DINSTR_PROF_COVMAP='__llvm_covmap' -DINSTR_PROF_COVFUN='__llvm_covfun' -DINSTR_PROF_ORDERFILE='__llvm_orderfile'
--- stderr -------------------------------
--- stderr -------------------------------
../coverage-llvmir/filecheck.testprog.txt:32:13: error: CHECK-SAME: expected string not found in input
CHECK-SAME: i8* bitcast ({ {{.*}} }* @__llvm_coverage_mapping to i8*)
<stdin>:93:30: note: scanning from here
<stdin>:93:30: note: scanning from here
@llvm.used = appending global [1 x i8*] [i8* getelementptr inbounds ([409 x i8], [409 x i8]* @__llvm_prf_nm, i32 0, i32 0)], section "llvm.metadata"
<stdin>:146:2: note: possible intended match here
<stdin>:146:2: note: possible intended match here
 %5 = bitcast { i8*, i64* }* %2 to %"core::fmt::Opaque"**

Input file: <stdin>
Input file: <stdin>
Check file: ../coverage-llvmir/filecheck.testprog.txt

-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          88: @__profd__RNCNvCsaweISYGdvTC_8testprog4mains0_0B3_ = private global { i64, i64, i64, i8*, i8*, i32, [2 x i16] } { i64 -3082815646967505383, i64 0, i64 sub (i64 ptrtoint ([1 x i64]* @__profc__RNCNvCsaweISYGdvTC_8testprog4mains0_0B3_ to i64), i64 ptrtoint ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNCNvCsaweISYGdvTC_8testprog4mains0_0B3_ to i64)), i8* null, i8* null, i32 1, [2 x i16] zeroinitializer }, section "__llvm_prf_data", comdat($__profc__RNCNvCsaweISYGdvTC_8testprog4mains0_0B3_), align 8 
          89: @__profc__RNvCsaweISYGdvTC_8testprog18will_not_be_called = private global [1 x i64] zeroinitializer, section "__llvm_prf_cnts", comdat, align 8 
          90: @__profd__RNvCsaweISYGdvTC_8testprog18will_not_be_called = private global { i64, i64, i64, i8*, i8*, i32, [2 x i16] } { i64 6148138824300301437, i64 0, i64 sub (i64 ptrtoint ([1 x i64]* @__profc__RNvCsaweISYGdvTC_8testprog18will_not_be_called to i64), i64 ptrtoint ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCsaweISYGdvTC_8testprog18will_not_be_called to i64)), i8* null, i8* null, i32 1, [2 x i16] zeroinitializer }, section "__llvm_prf_data", comdat($__profc__RNvCsaweISYGdvTC_8testprog18will_not_be_called), align 8 
          91: @__llvm_prf_nm = private constant [409 x i8] c"\96\03\00_RNvCsaweISYGdvTC_8testprog14will_be_called\01_RINvCsaweISYGdvTC_8testprog5printRReEB2_\01_RINvCsaweISYGdvTC_8testprog9wrap_withNCNvB2_4main0ReEB2_\01_RINvCsaweISYGdvTC_8testprog9wrap_withNCNvB2_4mains_0ReEB2_\01_RNvCsaweISYGdvTC_8testprog4main\01_RNCNvCsaweISYGdvTC_8testprog4main0B3_\01_RNCNvCsaweISYGdvTC_8testprog4mains_0B3_\01_RNCNvCsaweISYGdvTC_8testprog4mains0_0B3_\01_RNvCsaweISYGdvTC_8testprog18will_not_be_called", section "__llvm_prf_names", align 1 
          92: @llvm.compiler.used = appending global [9 x i8*] [i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCsaweISYGdvTC_8testprog14will_be_called to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RINvCsaweISYGdvTC_8testprog5printRReEB2_ to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RINvCsaweISYGdvTC_8testprog9wrap_withNCNvB2_4main0ReEB2_ to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RINvCsaweISYGdvTC_8testprog9wrap_withNCNvB2_4mains_0ReEB2_ to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCsaweISYGdvTC_8testprog4main to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNCNvCsaweISYGdvTC_8testprog4main0B3_ to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNCNvCsaweISYGdvTC_8testprog4mains_0B3_ to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNCNvCsaweISYGdvTC_8testprog4mains0_0B3_ to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCsaweISYGdvTC_8testprog18will_not_be_called to i8*)], section "llvm.metadata" 
          93: @llvm.used = appending global [1 x i8*] [i8* getelementptr inbounds ([409 x i8], [409 x i8]* @__llvm_prf_nm, i32 0, i32 0)], section "llvm.metadata" 
same:32'0                                  X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          94:  
same:32'0     ~
          95: ; <core::fmt::ArgumentV1>::new_display::<&&str> 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          96: ; Function Attrs: inlinehint nonlazybind uwtable 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          97: define internal { i8*, i64* } @_RINvMs1_NtCsfEnIF1XuLuh_4core3fmtNtB6_10ArgumentV111new_displayRReECsaweISYGdvTC_8testprog({ [0 x i8]*, i64 }** align 8 %x) unnamed_addr #0 { 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          98: start: 
same:32'0     ~~~~~~~
           .
           .
           .
         141:  store %"core::fmt::Opaque"* %4, %"core::fmt::Opaque"** %0, align 8 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         142:  %_5 = load %"core::fmt::Opaque"*, %"core::fmt::Opaque"** %0, align 8, !nonnull !3, !align !4, !noundef !3 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         143:  br label %bb2 
same:32'0     ~~~~~~~~~~~~~~~
         144:  
same:32'0     ~
         145: bb2: ; preds = %bb1 
same:32'0     ~~~~~~~~~~~~~~~~~~~~
         146:  %5 = bitcast { i8*, i64* }* %2 to %"core::fmt::Opaque"** 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
same:32'1      ?                                                         possible intended match
         147:  store %"core::fmt::Opaque"* %_5, %"core::fmt::Opaque"** %5, align 8 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         148:  %6 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %2, i32 0, i32 1 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         149:  %7 = bitcast i64** %6 to i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)** 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         150:  store i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)* %_3, i1 (%"core::fmt::Opaque"*, %"core::fmt::Formatter"*)** %7, align 8 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         151:  %8 = getelementptr inbounds { i8*, i64* }, { i8*, i64* }* %2, i32 0, i32 0 
same:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
>>>>>>
make: *** [Makefile:60: test_llvm_ir] Error 1


---- [run-make] src/test/run-make-fulldeps/coverage-reports stdout ----


error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/used_inline_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_inline_crate.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/unused_mod_helper.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/unused_mod_helper.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/doctest_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/doctest_crate.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/inline_always_with_dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/inline_always_with_dead_code.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/used_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_crate.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/generator.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/generator.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/generator || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/generator.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/generator.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/generator.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-generator
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-generator/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.generator.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.generator.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.generator.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.generator.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.generator.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# FIXME(richkadel): None of the Rust test source samples have the
# `// ignore-llvm-cov-show-diffs` anymore. This directive exists to work around a limitation
# with `llvm-cov show`. When reporting coverage for multiple instantiations of a generic function,
# with different type substitutions, `llvm-cov show` prints these in a non-deterministic order,
# breaking the `diff` comparision.
#
# A partial workaround is implemented below, with `diff --ignore-matching-lines=RE`
# to ignore each line prefixing each generic instantiation coverage code region.
#
# This workaround only works if the coverage counts are identical across all reported
# instantiations. If there is no way to ensure this, you may need to apply the
# `// ignore-llvm-cov-show-diffs` directive, and check for differences using the
# `.json` files to validate that results have not changed. (Until then, the JSON
# files are redundant, so there is no need to generate `expected_*.json` files or
# compare actual JSON results.)
diff -u --strip-trailing-cr --ignore-matching-lines='^  | .*::<.*>.*:$' --ignore-matching-lines='^  | <.*>::.*:$' \
 expected_show_coverage.generator.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.generator.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/generator.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/generator.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/generator.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/dead_code.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/dead_code || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/dead_code.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/dead_code.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-dead_code
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-dead_code/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.dead_code.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.dead_code.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.dead_code.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.dead_code.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.dead_code.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# FIXME(richkadel): None of the Rust test source samples have the
# `// ignore-llvm-cov-show-diffs` anymore. This directive exists to work around a limitation
# with `llvm-cov show`. When reporting coverage for multiple instantiations of a generic function,
# with different type substitutions, `llvm-cov show` prints these in a non-deterministic order,
# breaking the `diff` comparision.
#
# A partial workaround is implemented below, with `diff --ignore-matching-lines=RE`
# to ignore each line prefixing each generic instantiation coverage code region.
#
# This workaround only works if the coverage counts are identical across all reported
# instantiations. If there is no way to ensure this, you may need to apply the
# `// ignore-llvm-cov-show-diffs` directive, and check for differences using the
# `.json` files to validate that results have not changed. (Until then, the JSON
# files are redundant, so there is no need to generate `expected_*.json` files or
# compare actual JSON results.)
diff -u --strip-trailing-cr --ignore-matching-lines='^  | .*::<.*>.*:$' --ignore-matching-lines='^  | <.*>::.*:$' \
 expected_show_coverage.dead_code.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.dead_code.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/dead_code.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/dead_code.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/dead_code.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/if.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/if.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/if || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/if.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/if.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/if.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-if
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-if/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.if.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.if.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.if.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.if.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.if.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# FIXME(richkadel): None of the Rust test source samples have the
# `// ignore-llvm-cov-show-diffs` anymore. This directive exists to work around a limitation
# with `llvm-cov show`. When reporting coverage for multiple instantiations of a generic function,
# with different type substitutions, `llvm-cov show` prints these in a non-deterministic order,
# breaking the `diff` comparision.
#
# A partial workaround is implemented below, with `diff --ignore-matching-lines=RE`
# to ignore each line prefixing each generic instantiation coverage code region.
#
# This workaround only works if the coverage counts are identical across all reported
# instantiations. If there is no way to ensure this, you may need to apply the
# `// ignore-llvm-cov-show-diffs` directive, and check for differences using the
# `.json` files to validate that results have not changed. (Until then, the JSON
# files are redundant, so there is no need to generate `expected_*.json` files or
# compare actual JSON results.)
diff -u --strip-trailing-cr --ignore-matching-lines='^  | .*::<.*>.*:$' --ignore-matching-lines='^  | <.*>::.*:$' \
 expected_show_coverage.if.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.if.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/if.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/while_early_ret.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/while_early_ret.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/while_early_ret || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/while_early_ret.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/while_early_ret.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/while_early_ret.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-while_early_ret
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-while_early_ret/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.while_early_ret.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.while_early_ret.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.while_early_ret.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.while_early_ret.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.while_early_ret.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# FIXME(richkadel): None of the Rust test source samples have the
# `// ignore-llvm-cov-show-diffs` anymore. This directive exists to work around a limitation
# with `llvm-cov show`. When reporting coverage for multiple instantiations of a generic function,
# with different type substitutions, `llvm-cov show` prints these in a non-deterministic order,
# breaking the `diff` comparision.
#
# A partial workaround is implemented below, with `diff --ignore-matching-lines=RE`
# to ignore each line prefixing each generic instantiation coverage code region.
#
# This workaround only works if the coverage counts are identical across all reported
# instantiations. If there is no way to ensure this, you may need to apply the
# `// ignore-llvm-cov-show-diffs` directive, and check for differences using the
# `.json` files to validate that results have not changed. (Until then, the JSON
# files are redundant, so there is no need to generate `expected_*.json` files or
# compare actual JSON results.)
diff -u --strip-trailing-cr --ignore-matching-lines='^  | .*::<.*>.*:$' --ignore-matching-lines='^  | <.*>::.*:$' \
 expected_show_coverage.while_early_ret.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.while_early_ret.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/while_early_ret.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/while_early_ret.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/while_early_ret.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/assert.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/assert.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/assert || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/assert.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
does 1 + 1 = 2?
does 1 + 1 = 2?
does 1 + 1 = 2?
does 1 + 1 = 3?
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/assert.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/assert.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-assert
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-assert/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.assert.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.assert.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.assert.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.assert.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.assert.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# FIXME(richkadel): None of the Rust test source samples have the
# `// ignore-llvm-cov-show-diffs` anymore. This directive exists to work around a limitation
# with `llvm-cov show`. When reporting coverage for multiple instantiations of a generic function,
# with different type substitutions, `llvm-cov show` prints these in a non-deterministic order,
# breaking the `diff` comparision.
---
called and covered
called but not covered
called and covered
called and covered
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/no_cov_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/no_cov_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-no_cov_crate
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-no_cov_crate/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.no_cov_crate.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.no_cov_crate.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.no_cov_crate.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.no_cov_crate.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.no_cov_crate.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# FIXME(richkadel): None of the Rust test source samples have the
# `// ignore-llvm-cov-show-diffs` anymore. This directive exists to work around a limitation
# with `llvm-cov show`. When reporting coverage for multiple instantiations of a generic function,
# with different type substitutions, `llvm-cov show` prints these in a non-deterministic order,
# breaking the `diff` comparision.
#
# A partial workaround is implemented below, with `diff --ignore-matching-lines=RE`
# to ignore each line prefixing each generic instantiation coverage code region.
#
# This workaround only works if the coverage counts are identical across all reported
# instantiations. If there is no way to ensure this, you may need to apply the
# `// ignore-llvm-cov-show-diffs` directive, and check for differences using the
# `.json` files to validate that results have not changed. (Until then, the JSON
# files are redundant, so there is no need to generate `expected_*.json` files or
# compare actual JSON results.)
diff -u --strip-trailing-cr --ignore-matching-lines='^  | .*::<.*>.*:$' --ignore-matching-lines='^  | <.*>::.*:$' \
 expected_show_coverage.no_cov_crate.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.no_cov_crate.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/no_cov_crate.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/no_cov_crate.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/no_cov_crate.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/issue-93054.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/issue-93054.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/issue-93054 || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/issue-93054.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/issue-93054.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/issue-93054.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-issue-93054
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054 \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-issue-93054/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.issue-93054.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.issue-93054.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.issue-93054.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.issue-93054.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.issue-93054.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# FIXME(richkadel): None of the Rust test source samples have the
# `// ignore-llvm-cov-show-diffs` anymore. This directive exists to work around a limitation
# with `llvm-cov show`. When reporting coverage for multiple instantiations of a generic function,
# with different type substitutions, `llvm-cov show` prints these in a non-deterministic order,
# breaking the `diff` comparision.
#
# A partial workaround is implemented below, with `diff --ignore-matching-lines=RE`
# to ignore each line prefixing each generic instantiation coverage code region.
#
# This workaround only works if the coverage counts are identical across all reported
# instantiations. If there is no way to ensure this, you may need to apply the
# `// ignore-llvm-cov-show-diffs` directive, and check for differences using the
# `.json` files to validate that results have not changed. (Until then, the JSON
# files are redundant, so there is no need to generate `expected_*.json` files or
# compare actual JSON results.)
diff -u --strip-trailing-cr --ignore-matching-lines='^  | .*::<.*>.*:$' --ignore-matching-lines='^  | <.*>::.*:$' \
 expected_show_coverage.issue-93054.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.issue-93054.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/issue-93054.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/issue-93054.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/issue-93054.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/uses_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/uses_crate || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/uses_crate.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
used_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_only_from_this_lib_crate_generic_function with [5, 6, 7, 8]
used_only_from_this_lib_crate_generic_function with "used ONLY from library used_crate.rs"
used_only_from_bin_crate_generic_function with [1, 2, 3, 4]
used_only_from_bin_crate_generic_function with "used from bin uses_crate.rs"
used_from_bin_crate_and_lib_crate_generic_function with [1, 2, 3, 4]
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "interesting?"
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/uses_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-uses_crate
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-uses_crate/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.uses_crate.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.uses_crate.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.uses_crate.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.uses_crate.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.uses_crate.txt
# Compare the show coverage output (`--bless` refreshes `typical` files).
#
# FIXME(richkadel): None of the Rust test source samples have the
# `// ignore-llvm-cov-show-diffs` anymore. This directive exists to work around a limitation
# with `llvm-cov show`. When reporting coverage for multiple instantiations of a generic function,
# with different type substitutions, `llvm-cov show` prints these in a non-deterministic order,
# breaking the `diff` comparision.
#
# A partial workaround is implemented below, with `diff --ignore-matching-lines=RE`
# to ignore each line prefixing each generic instantiation coverage code region.
#
# This workaround only works if the coverage counts are identical across all reported
# instantiations. If there is no way to ensure this, you may need to apply the
# `// ignore-llvm-cov-show-diffs` directive, and check for differences using the
# `.json` files to validate that results have not changed. (Until then, the JSON
# files are redundant, so there is no need to generate `expected_*.json` files or
# compare actual JSON results.)
diff -u --strip-trailing-cr --ignore-matching-lines='^  | .*::<.*>.*:$' --ignore-matching-lines='^  | <.*>::.*:$' \
 expected_show_coverage.uses_crate.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.uses_crate.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/uses_crate.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs'; \
 )
 )
--- expected_show_coverage.uses_crate.txt 2022-04-10 23:51:52.594406192 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/actual_show_coverage.uses_crate.txt 2022-04-11 00:29:37.810775682 +0000
@@ -1,148 +0,0 @@
-    1|       |#![allow(unused_assignments, unused_variables)]
-    2|       |// compile-flags: -C opt-level=3 # validates coverage now works with optimizations
-    3|       |use std::fmt::Debug;
-    4|       |
-    5|      1|pub fn used_function() {
-    6|      1|    // Initialize test constants in a way that cannot be determined at compile time, to ensure
-    7|      1|    // rustc and LLVM cannot optimize out statements (or coverage counters) downstream from
-    8|      1|    // dependent conditions.
-    9|      1|    let is_true = std::env::args().len() == 1;
-   10|      1|    let mut countdown = 0;
-   11|      1|    if is_true {
-   12|      1|        countdown = 10;
-   13|      1|    }
-   14|      1|    use_this_lib_crate();
-   15|      1|}
-   16|       |
-   16|       |
-   17|      2|pub fn used_only_from_bin_crate_generic_function<T: Debug>(arg: T) {
-   18|      2|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
-   19|      2|}
-  | used_crate::used_only_from_bin_crate_generic_function::<&str>:
-  | used_crate::used_only_from_bin_crate_generic_function::<&str>:
-  |   17|      1|pub fn used_only_from_bin_crate_generic_function<T: Debug>(arg: T) {
-  |   18|      1|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
-  |   19|      1|}
-  ------------------
-  | used_crate::used_only_from_bin_crate_generic_function::<&alloc::vec::Vec<i32>>:
-  |   17|      1|pub fn used_only_from_bin_crate_generic_function<T: Debug>(arg: T) {
-  |   18|      1|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
-  |   19|      1|}
-  ------------------
-  | Unexecuted instantiation: used_crate::used_only_from_bin_crate_generic_function::<_>
-  ------------------
-   20|       |// Expect for above function: `Unexecuted instantiation` (see below)
-   21|      2|pub fn used_only_from_this_lib_crate_generic_function<T: Debug>(arg: T) {
-   22|      2|    println!("used_only_from_this_lib_crate_generic_function with {:?}", arg);
-   23|      2|}
-  | used_crate::used_only_from_this_lib_crate_generic_function::<&str>:
-  | used_crate::used_only_from_this_lib_crate_generic_function::<&str>:
-  |   21|      1|pub fn used_only_from_this_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   22|      1|    println!("used_only_from_this_lib_crate_generic_function with {:?}", arg);
-  |   23|      1|}
-  ------------------
-  | used_crate::used_only_from_this_lib_crate_generic_function::<alloc::vec::Vec<i32>>:
-  |   21|      1|pub fn used_only_from_this_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   22|      1|    println!("used_only_from_this_lib_crate_generic_function with {:?}", arg);
-  |   23|      1|}
-   24|       |
-   24|       |
-   25|      2|pub fn used_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-   26|      2|    println!("used_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-   27|      2|}
-  ------------------
-  | used_crate::used_from_bin_crate_and_lib_crate_generic_function::<alloc::vec::Vec<i32>>:
-  |   25|      1|pub fn used_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   26|      1|    println!("used_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-  |   27|      1|}
-  | used_crate::used_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  | used_crate::used_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  |   25|      1|pub fn used_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   26|      1|    println!("used_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-  |   27|      1|}
-   28|       |
-   28|       |
-   29|      2|pub fn used_with_same_type_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-   30|      2|    println!("used_with_same_type_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-   31|      2|}
-  | used_crate::used_with_same_type_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  | used_crate::used_with_same_type_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  |   29|      1|pub fn used_with_same_type_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   30|      1|    println!("used_with_same_type_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-  |   31|      1|}
-  | used_crate::used_with_same_type_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  | used_crate::used_with_same_type_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  |   29|      1|pub fn used_with_same_type_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   30|      1|    println!("used_with_same_type_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-  |   31|      1|}
-   32|       |
-   32|       |
-   33|      0|pub fn unused_generic_function<T: Debug>(arg: T) {
-   34|      0|    println!("unused_generic_function with {:?}", arg);
-   35|      0|}
-   36|       |
-   37|      0|pub fn unused_function() {
-   38|      0|    let is_true = std::env::args().len() == 1;
-   39|      0|    let mut countdown = 2;
-   40|      0|    if !is_true {
-   41|      0|        countdown = 20;
-   42|      0|    }
-   43|      0|}
-   44|       |
-   45|      0|fn unused_private_function() {
-   46|      0|    let is_true = std::env::args().len() == 1;
-   47|      0|    let mut countdown = 2;
-   48|      0|    if !is_true {
-   49|      0|        countdown = 20;
-   50|      0|    }
-   51|      0|}
-   52|       |
-   53|      1|fn use_this_lib_crate() {
-   54|      1|    used_from_bin_crate_and_lib_crate_generic_function("used from library used_crate.rs");
-   55|      1|    used_with_same_type_from_bin_crate_and_lib_crate_generic_function(
-   56|      1|        "used from library used_crate.rs",
-   57|      1|    );
-   58|      1|    let some_vec = vec![5, 6, 7, 8];
-   59|      1|    used_only_from_this_lib_crate_generic_function(some_vec);
-   60|      1|    used_only_from_this_lib_crate_generic_function("used ONLY from library used_crate.rs");
-   61|      1|}
-   62|       |
-   63|       |// FIXME(#79651): "Unexecuted instantiation" errors appear in coverage results,
-   64|       |// for example:
-   65|       |//
-   66|       |// | Unexecuted instantiation: used_crate::used_only_from_bin_crate_generic_function::<_>
-   67|       |//
-   68|       |// These notices appear when `llvm-cov` shows instantiations. This may be a
-   69|       |// default option, but it can be suppressed with:
-   70|       |//
-   71|       |// 