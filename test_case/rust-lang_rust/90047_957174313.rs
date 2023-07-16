plain
status: exit status: 2
command: "make"
stdout:
------------------------------------------
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/used_inline_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_inline_crate.rs ) \
  --crate-type rlib -Zinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/doctest_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/doctest_crate.rs ) \
  --crate-type rlib -Zinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/used_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_crate.rs ) \
  --crate-type rlib -Zinstrument-coverage
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/dead_code.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
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
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
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
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs)' \
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
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/uses_inline_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_inline_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_inline_crate.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/uses_inline_crate || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/uses_inline_crate.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
used_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_with_same_type_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
used_only_from_this_lib_crate_generic_function with [5, 6, 7, 8]
used_only_from_this_lib_crate_generic_function with "used ONLY from library used_crate.rs"
used_from_bin_crate_and_lib_crate_generic_function with "used from library used_crate.rs"
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
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_inline_crate-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/uses_inline_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_inline_crate.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-uses_inline_crate
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_inline_crate*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_inline_crate.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs)' \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_inline_crate.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_inline_crate \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-uses_inline_crate/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.uses_inline_crate.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.uses_inline_crate.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.uses_inline_crate.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.uses_inline_crate.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.uses_inline_crate.txt
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
 expected_show_coverage.uses_inline_crate.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.uses_inline_crate.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/uses_inline_crate.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/uses_inline_crate.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/uses_inline_crate.rs'; \
 )
 )
--- expected_show_coverage.uses_inline_crate.txt 2021-11-02 07:00:58.662289784 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/actual_show_coverage.uses_inline_crate.txt 2021-11-02 07:46:22.073670936 +0000
@@ -18,18 +18,17 @@
    17|      1|}
    18|       |
    19|       |#[inline(always)]
-   20|      1|pub fn used_inline_function() {
-   21|      1|    // Initialize test constants in a way that cannot be determined at compile time, to ensure
-   22|      1|    // rustc and LLVM cannot optimize out statements (or coverage counters) downstream from
-   23|      1|    // dependent conditions.
-   24|      1|    let is_true = std::env::args().len() == 1;
-   25|      1|    let mut countdown = 0;
-   26|      1|    if is_true {
-   27|      1|        countdown = 10;
-   28|      1|    }
-   29|      1|    use_this_lib_crate();
-   30|      1|}
-   30|      1|}
+   20|       |pub fn used_inline_function() {
+   21|       |    // Initialize test constants in a way that cannot be determined at compile time, to ensure
+   22|       |    // rustc and LLVM cannot optimize out statements (or coverage counters) downstream from
+   23|       |    // dependent conditions.
+   24|       |    let is_true = std::env::args().len() == 1;
+   25|       |    let mut countdown = 0;
+   26|       |    if is_true {
+   27|       |        countdown = 10;
+   29|       |    use_this_lib_crate();
+   30|       |}
    31|       |
    32|       |
    32|       |
    33|       |
@@ -38,22 +37,9 @@
    37|       |
    37|       |
    38|       |#[inline(always)]
-   39|      2|pub fn used_only_from_bin_crate_generic_function<T: Debug>(arg: T) {
-   40|      2|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
-   41|      2|}
-  ------------------
-  | used_inline_crate::used_only_from_bin_crate_generic_function::<&str>:
-  |   39|      1|pub fn used_only_from_bin_crate_generic_function<T: Debug>(arg: T) {
-  |   40|      1|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
-  |   41|      1|}
-  ------------------
-  | used_inline_crate::used_only_from_bin_crate_generic_function::<&alloc::vec::Vec<i32>>:
-  |   39|      1|pub fn used_only_from_bin_crate_generic_function<T: Debug>(arg: T) {
-  |   40|      1|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
-  |   41|      1|}
-  ------------------
-  | Unexecuted instantiation: used_inline_crate::used_only_from_bin_crate_generic_function::<_>
-  ------------------
+   39|      0|pub fn used_only_from_bin_crate_generic_function<T: Debug>(arg: T) {
+   40|      0|    println!("used_only_from_bin_crate_generic_function with {:?}", arg);
+   41|      0|}
    42|       |// Expect for above function: `Unexecuted instantiation` (see notes in `used_crate.rs`)
    43|       |
    44|       |#[inline(always)]
@@ -66,43 +52,21 @@
   |   46|      2|    println!("used_only_from_this_lib_crate_generic_function with {:?}", arg);
   |   47|      2|}
   ------------------
-  | used_inline_crate::used_only_from_this_lib_crate_generic_function::<alloc::vec::Vec<i32>>:
+  | used_inline_crate::used_only_from_this_lib_crate_generic_function::<&str>:
   |   45|      2|pub fn used_only_from_this_lib_crate_generic_function<T: Debug>(arg: T) {
   |   46|      2|    println!("used_only_from_this_lib_crate_generic_function with {:?}", arg);
   |   47|      2|}
    48|       |
    48|       |
    49|       |#[inline(always)]
-   50|      3|pub fn used_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-   51|      3|    println!("used_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-   52|      3|}
-  ------------------
-  | used_inline_crate::used_from_bin_crate_and_lib_crate_generic_function::<alloc::vec::Vec<i32>>:
-  |   50|      1|pub fn used_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   51|      1|    println!("used_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-  |   52|      1|}
-  | used_inline_crate::used_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  | used_inline_crate::used_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  |   50|      2|pub fn used_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   51|      2|    println!("used_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-  |   52|      2|}
-  ------------------
+   50|      2|pub fn used_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
+   51|      2|    println!("used_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
+   52|      2|}
    53|       |
    54|       |#[inline(always)]
-   55|      3|pub fn used_with_same_type_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-   56|      3|    println!("used_with_same_type_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-   57|      3|}
-  ------------------
-  | used_inline_crate::used_with_same_type_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  |   55|      1|pub fn used_with_same_type_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   56|      1|    println!("used_with_same_type_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-  |   57|      1|}
-  ------------------
-  | used_inline_crate::used_with_same_type_from_bin_crate_and_lib_crate_generic_function::<&str>:
-  |   55|      2|pub fn used_with_same_type_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
-  |   56|      2|    println!("used_with_same_type_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
-  |   57|      2|}
-  ------------------
+   55|      2|pub fn used_with_same_type_from_bin_crate_and_lib_crate_generic_function<T: Debug>(arg: T) {
+   56|      2|    println!("used_with_same_type_from_bin_crate_and_lib_crate_generic_function with {:?}", arg);
+   57|      2|}
    58|       |
    59|       |#[inline(always)]
    60|      0|pub fn unused_generic_function<T: Debug>(arg: T) {
------------------------------------------
stderr:
------------------------------------------
warning: function is never used: `unused_private_function`
---
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/uses_inline_crate.rs
make: *** [Makefile:117: uses_inline_crate] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/coverage-reports

test result: FAILED. 220 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out; finished in 33.18s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:43:55
