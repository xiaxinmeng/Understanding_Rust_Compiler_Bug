plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/used_inline_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_inline_crate.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/unused_mod_helper.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/unused_mod_helper.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/doctest_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/doctest_crate.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/inline_always_with_dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/inline_always_with_dead_code.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lib/used_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_crate.rs ) \
  --crate-type rlib -Cinstrument-coverage
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/generator.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/generator.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/generator || \
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
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/generator.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/generator.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-generator
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/generator \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-generator/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.generator.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.generator.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.generator.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.generator.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.generator.txt
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
 expected_show_coverage.generator.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.generator.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/generator.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/generator.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/generator.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/dead_code.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/dead_code || \
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
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/dead_code.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/dead_code.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-dead_code
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/dead_code \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-dead_code/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.dead_code.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.dead_code.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.dead_code.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.dead_code.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.dead_code.txt
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
 expected_show_coverage.dead_code.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.dead_code.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/dead_code.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/dead_code.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/dead_code.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/continue.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/continue.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/continue.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/continue || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/continue.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/continue-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/continue.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/continue.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-continue
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/continue*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/continue.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/continue.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/continue \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-continue/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.continue.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.continue.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.continue.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.continue.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.continue.txt
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
 expected_show_coverage.continue.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.continue.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/continue.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/continue.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/continue.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/if.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/if.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/if || \
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
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/if.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/if.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-if
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/if \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-if/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.if.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.if.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.if.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.if.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.if.txt
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
 expected_show_coverage.if.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.if.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/if.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/if.rs'; \
 )
# Compile the test program with coverage instrumentation
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/while_early_ret.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/while_early_ret.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/while_early_ret || \
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
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/while_early_ret.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/while_early_ret.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-while_early_ret
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/while_early_ret \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-while_early_ret/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.while_early_ret.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.while_early_ret.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.while_early_ret.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.while_early_ret.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.while_early_ret.txt
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
 expected_show_coverage.while_early_ret.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.while_early_ret.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/while_early_ret.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/while_early_ret.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/while_early_ret.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/assert.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/assert.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/assert || \
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
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/assert.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/assert.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-assert
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/assert \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-assert/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.assert.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.assert.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.assert.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.assert.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.assert.txt
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
---
called and covered
called but not covered
called and covered
called and covered
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/no_cov_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/no_cov_crate.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-no_cov_crate
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/no_cov_crate \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-no_cov_crate/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.no_cov_crate.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.no_cov_crate.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.no_cov_crate.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.no_cov_crate.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.no_cov_crate.txt
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
 expected_show_coverage.no_cov_crate.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.no_cov_crate.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/no_cov_crate.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/no_cov_crate.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/no_cov_crate.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/issue-93054.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/issue-93054.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/issue-93054 || \
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
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/issue-93054.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/issue-93054.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-issue-93054
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-93054 \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-issue-93054/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.issue-93054.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.issue-93054.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.issue-93054.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.issue-93054.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.issue-93054.txt
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
 expected_show_coverage.issue-93054.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.issue-93054.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/issue-93054.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/issue-93054.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/issue-93054.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/uses_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_crate.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/uses_crate || \
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
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/uses_crate.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/uses_crate.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-uses_crate
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/uses_crate \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-uses_crate/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.uses_crate.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.uses_crate.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.uses_crate.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.uses_crate.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.uses_crate.txt
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
 expected_show_coverage.uses_crate.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.uses_crate.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/uses_crate.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/lazy_boolean.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lazy_boolean.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/lazy_boolean.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/lazy_boolean || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/lazy_boolean.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/lazy_boolean-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/lazy_boolean.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lazy_boolean.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-lazy_boolean
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/lazy_boolean*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/lazy_boolean.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/lazy_boolean.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/lazy_boolean \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-lazy_boolean/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.lazy_boolean.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.lazy_boolean.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.lazy_boolean.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.lazy_boolean.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.lazy_boolean.txt
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
 expected_show_coverage.lazy_boolean.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.lazy_boolean.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/lazy_boolean.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/lazy_boolean.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/lazy_boolean.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/issue-85461.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/issue-85461.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-85461.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/issue-85461 || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/issue-85461.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-85461-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/issue-85461.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/issue-85461.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-issue-85461
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-85461*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-85461.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-85461.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/issue-85461 \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-issue-85461/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.issue-85461.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.issue-85461.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.issue-85461.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.issue-85461.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.issue-85461.txt
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
 expected_show_coverage.issue-85461.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.issue-85461.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/issue-85461.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/issue-85461.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/issue-85461.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/unused.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/unused.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/unused.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/unused || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/unused.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
  )
# Run it through rustdoc as well to cover doctests.
# `%p` is the pid, and `%m` the binary signature. We suspect that the pid alone
# might result in overwritten files and failed tests, as rustdoc spawns each
# doctest as its own process, so make sure the filename is as unique as possible.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/unused-%p-%m.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib --crate-name workaround_for_79771 --test ../coverage/unused.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/unused.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage \
  -Z unstable-options --persist-doctests=/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-unused
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Postprocess the profiling data so it can be used by the llvm-cov tool
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge --sparse \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/unused*.profraw \
  -o "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/unused.profdata
# Generate a coverage report using `llvm-cov show`.
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-cov show \
  --debug \
  --ignore-filename-regex='(uses_crate.rs|uses_inline_crate.rs|unused_mod.rs)' \
  --compilation-dir=. \
  --Xdemangler="/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" \
  --show-line-counts-or-regions \
  --instr-profile="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/unused.profdata \
  "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/unused \
  $( for file in /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/rustdoc-unused/*/rust_out; do [ -x "$file" ] && printf "%s %s " -object $file; done ) \
 2> "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.unused.txt \
 | "/usr/bin/python3" ../coverage-reports/normalize_paths.py \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.unused.txt || \
( status=$? ; \
 >&2 cat "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.unused.txt ; \
 exit $status \
)
# The first line (beginning with "Args:" contains hard-coded, build-specific
# file paths. Strip that line and keep the remaining lines with counter debug
# data.
tail -n +2 "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/show_coverage_stderr.unused.txt \
 > "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage_counters.unused.txt
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
 expected_show_coverage.unused.txt "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.unused.txt || \
 ( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/unused.rs && \
  >&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/unused.rs' \
 ) || \
 ( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/unused.rs'; \
 )
# Compile the test program with coverage instrumentation
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports  ../coverage/panic_unwind.rs \
  $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/panic_unwind.rs ) \
  -L "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports" -Cinstrument-coverage
# Run it in order to generate some profiling data,
# with `LLVM_PROFILE_FILE=<profdata_file>` environment variable set to
# output the coverage stats for this run.
LLVM_PROFILE_FILE="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports"/panic_unwind.profraw \
  LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/panic_unwind || \
  ( \
   status=$?; \
   grep -q "^\/\/ expect-exit-status-$status" ../coverage/panic_unwind.rs || \
   ( >&2 echo "program exited with an unexpected exit status: $status"; \
   ) \
  )
Don't Panic
Don't Panic
Don't Panic
---

Error: 1
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `3`: the argument was wrong', ../coverage/assert.rs:6:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: function is never used: `do_not_add_coverage_not_called`
  --> ../coverage/no_cov_crate.rs:15:4
   |
15 | fn do_not_add_coverage_not_called() {
   |
   = note: `#[warn(dead_code)]` on by default

warning: function is never used: `add_coverage_not_called`
warning: function is never used: `add_coverage_not_called`
  --> ../coverage/no_cov_crate.rs:27:4
27 | fn add_coverage_not_called() {
   |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: 2 warnings emitted
warning: 2 warnings emitted

warning: unreachable statement
  --> ../coverage/issue-93054.rs:12:9
   |
11 |         match self { }
   |         -------------- any code following this expression is unreachable
12 |         make().map(|never| match never { });
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
   = note: `#[warn(unreachable_code)]` on by default

warning: enum is never used: `Never`
 --> ../coverage/issue-93054.rs:7:6
 --> ../coverage/issue-93054.rs:7:6
  |
7 | enum Never { }
  |
  = note: `#[warn(dead_code)]` on by default

warning: associated function is never used: `foo`
---

warning: function is never used: `foo2`
  --> ../coverage/issue-93054.rs:20:10
   |
20 | async fn foo2(never: Never) {


warning: function is never used: `make`
  --> ../coverage/issue-93054.rs:24:4
   |
24 | fn make() -> Option<Never> {

warning: 6 warnings emitted

warning: unused variable: `x`
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

thread 'main' panicked at 'panics', ../coverage/panic_unwind.rs:7:9
thread 'main' panicked at 'panics', ../coverage/panic_unwind.rs:7:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
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
warning: function is never used: `never_called_function`
 --> ../coverage/lib/unused_mod_helper.rs:1:8
  |
1 | pub fn never_called_function() {
1 | pub fn never_called_function() {
  |        ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

Error: ()
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

diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/closure.rs
make: *** [Makefile:123: closure] Error 1



failures:
