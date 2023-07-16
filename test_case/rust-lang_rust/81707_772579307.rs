plain
status: exit code: 2
command: "make"
stdout:
------------------------------------------
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/doctest_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/doctest_crate.rs && echo "--edition=2018" ) \
  --crate-type rlib -Zinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/used_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/used_crate.rs && echo "--edition=2018" ) \
  --crate-type rlib -Zinstrument-coverage
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/dead_code.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/dead_code.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code-%p.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/dead_code || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/dead_code.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code-%p.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/dead_code.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/dead_code.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-dead_code
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code-*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex=uses_crate.rs \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-dead_code/*/rust_out; do [[ -x $file ]] && printf "%s %s " -object $file; done ) \
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
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/if.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if-%p.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/if || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/if.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if-%p.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/if.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/if.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-if
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if-*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex=uses_crate.rs \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-if/*/rust_out; do [[ -x $file ]] && printf "%s %s " -object $file; done ) \
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
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/yield.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/yield.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield-%p.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/yield || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/yield.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield-%p.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/yield.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/yield.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-yield
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield-*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex=uses_crate.rs \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-yield/*/rust_out; do [[ -x $file ]] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.yield.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.yield.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.yield.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.yield.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.yield.txt
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
 expected_show_coverage.yield.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.yield.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/yield.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/yield.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/yield.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/doctest.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/doctest.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/doctest-%p.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/doctest || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/doctest.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/doctest-%p.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/doctest.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/doctest.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-doctest
running 5 tests
test ../coverage/doctest.rs - main (line 68) ... ok
test ../coverage/doctest.rs - (line 16) ... ok
test ../coverage/doctest.rs - (line 44) ... ok
test ../coverage/doctest.rs - (line 44) ... ok
test ../coverage/doctest.rs - (line 5) ... ok
test ../coverage/doctest.rs - (line 22) ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s

# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/doctest-*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/doctest.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex=uses_crate.rs \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/doctest.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/doctest \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-doctest/*/rust_out; do [[ -x $file ]] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.doctest.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.doctest.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.doctest.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.doctest.txt \
 > "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.doctest.txt
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
 expected_show_coverage.doctest.txt "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.doctest.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/doctest.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/doctest.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/doctest.rs'; \
 )
 )
--- expected_show_coverage.doctest.txt 2021-02-03 14:31:10.386036180 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/actual_show_coverage.doctest.txt 2021-02-03 15:06:58.191162553 +0000
@@ -1,65 +1,61 @@
-../coverage/doctest.rs:
     1|       |//! This test ensures that code from doctests is properly re-mapped.
     2|       |//! See <https://github.com/rust-lang/rust/issues/79417> for more info.
     3|       |//!
     4|       |//! Just some random code:
-    5|      1|//! 