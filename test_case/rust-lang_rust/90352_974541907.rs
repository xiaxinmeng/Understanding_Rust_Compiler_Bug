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
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/if.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/if.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
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
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
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
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs)' \
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
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/yield.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/yield.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/yield || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/yield.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/yield.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/yield.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-yield
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield*.profraw \
  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs)' \
  --Xdemangler="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield.profdata \
  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/yield \
  $( for file in /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-yield/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
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
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/while_early_ret.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/while_early_ret.rs ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
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
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
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
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs)' \
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
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage
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
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Zinstrument-coverage \
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
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs)' \
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
#
# A partial workaround is implemented below, with `diff --ignore-matching-lines=RE`
# to ignore each line prefixing each generic instantiation coverage code region.
#
# This workaround only works if the coverage counts are identical across all reported
---

Error: 1
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `3`: the argument was wrong', ../coverage/assert.rs:6:5
Error: 1
Error: 1
warning: function is never used: `do_not_add_coverage_not_called`
  --> ../coverage/no_cov_crate.rs:15:4
   |
15 | fn do_not_add_coverage_not_called() {
   |
   = note: `#[warn(dead_code)]` on by default


warning: function is never used: `add_coverage_not_called`
  --> ../coverage/no_cov_crate.rs:27:4
27 | fn add_coverage_not_called() {
   |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: 2 warnings emitted
warning: 2 warnings emitted

warning: unused variable: `x`
 --> ../coverage/unused.rs:1:11
  |
1 | fn foo<T>(x: T) {
  |           ^ help: if this is intentional, prefix it with an underscore: `_x`
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
 --> ../coverage/unused.rs:9:28
 --> ../coverage/unused.rs:9:28
  |
9 | fn unused_template_func<T>(x: T) {
  |                            ^ help: if this is intentional, prefix it with an underscore: `_x`

warning: value assigned to `a` is never read
  --> ../coverage/unused.rs:19:9
19 |         a += 1;
   |         ^
   |
   = note: `#[warn(unused_assignments)]` on by default
   = note: `#[warn(unused_assignments)]` on by default
   = help: maybe it is overwritten before being read?

warning: value assigned to `a` is never read
  --> ../coverage/unused.rs:25:9
25 |         a += 1;
   |         ^
   |
   |
   = help: maybe it is overwritten before being read?

warning: value assigned to `a` is never read
  --> ../coverage/unused.rs:31:9
31 |         a += 1;
   |         ^
   |
   |
   = help: maybe it is overwritten before being read?
warning: function is never used: `unused_template_func`
 --> ../coverage/unused.rs:9:4
  |
  |
9 | fn unused_template_func<T>(x: T) {
  |
  = note: `#[warn(dead_code)]` on by default

warning: function is never used: `unused_func`
warning: function is never used: `unused_func`
  --> ../coverage/unused.rs:17:4
   |
17 | fn unused_func(mut a: u32) {

warning: function is never used: `unused_func2`
  --> ../coverage/unused.rs:23:4
   |
   |
23 | fn unused_func2(mut a: u32) {

warning: function is never used: `unused_func3`
  --> ../coverage/unused.rs:29:4
   |
   |
29 | fn unused_func3(mut a: u32) {

warning: unused logical operation that must be used
 --> ../coverage/unused.rs:4:9
  |
  |
4 |         i != 0 || i != 0;
  |
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
  |
4 |         let _ = i != 0 || i != 0;

warning: unused logical operation that must be used
  --> ../coverage/unused.rs:12:9
   |
   |
12 |         i != 0 || i != 0;
   |
help: use `let _ = ...` to ignore the resulting value
   |
   |
12 |         let _ = i != 0 || i != 0;

warning: 11 warnings emitted


thread 'main' panicked at 'attempt to add with overflow', ../coverage/overflow.rs:10:18
warning: unused `Result` that must be used
  --> ../coverage/closure_macro_async.rs:44:5
   |
   |
44 |     executor::block_on(test());
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled
warning: 1 warning emitted

thread 'main' panicked at 'assertion failed: `(left == right)`
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `try and succeed`,
 right: `try and succeed`: this assert should fail', ../coverage/issue-84561.rs:123:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: unused imports: `future::Future`, `marker::Send`, `pin::Pin`
 --> ../coverage/async2.rs:4:5
4 |     future::Future,
  |     ^^^^^^^^^^^^^^
5 |     marker::Send,
  |     ^^^^^^^^^^^^
  |     ^^^^^^^^^^^^
6 |     pin::Pin,
  |     ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: 1 warning emitted

diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/loops_branches.rs
make: *** [Makefile:117: loops_branches] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/coverage-reports

test result: FAILED. 221 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out; finished in 33.31s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:41:30
