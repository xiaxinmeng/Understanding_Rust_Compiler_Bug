plain
status: exit code: 2
command: "make"
stdout:
------------------------------------------
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/doctest_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/doctest_crate.rs && echo "--edition=2018" ) \
  --crate-type rlib \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "doctest_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest_crate/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.doctest_crate/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest_crate/
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/used_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lib/used_crate.rs && echo "--edition=2018" ) \
  --crate-type rlib \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "used_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_crate/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.used_crate/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_crate/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/dead_code.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/dead_code.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.dead_code
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.dead_code/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "dead_code" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.dead_code
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.dead_code/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.dead_code/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.dead_code/
diff -u --strip-trailing-cr -r expected_mir_dump.dead_code/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.dead_code/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/if.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/if.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "if" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if/
diff -u --strip-trailing-cr -r expected_mir_dump.if/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/yield.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/yield.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.yield
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.yield/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "yield" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.yield
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.yield/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.yield/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.yield/
diff -u --strip-trailing-cr -r expected_mir_dump.yield/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.yield/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/doctest.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/doctest.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "doctest" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.doctest/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest/
diff -u --strip-trailing-cr -r expected_mir_dump.doctest/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.doctest/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/while_early_ret.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/while_early_ret.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while_early_ret
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while_early_ret/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "while_early_ret" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while_early_ret
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while_early_ret/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.while_early_ret/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while_early_ret/
diff -u --strip-trailing-cr -r expected_mir_dump.while_early_ret/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.while_early_ret/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/assert.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/assert.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "assert" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.assert/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert/
diff -u --strip-trailing-cr -r expected_mir_dump.assert/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.assert/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/drop_trait.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/drop_trait.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.drop_trait
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.drop_trait/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "drop_trait" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.drop_trait
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.drop_trait/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.drop_trait/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.drop_trait/
diff -u --strip-trailing-cr -r expected_mir_dump.drop_trait/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.drop_trait/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/inner_items.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/inner_items.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.inner_items
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.inner_items/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "inner_items" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inner_items
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inner_items/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.inner_items/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inner_items/
diff -u --strip-trailing-cr -r expected_mir_dump.inner_items/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.inner_items/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/uses_crate.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/uses_crate.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.uses_crate
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.uses_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "uses_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.uses_crate
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.uses_crate/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.uses_crate/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.uses_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.uses_crate/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.uses_crate/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lazy_boolean.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/lazy_boolean.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.lazy_boolean
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.lazy_boolean/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "lazy_boolean" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.lazy_boolean
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.lazy_boolean/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.lazy_boolean/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.lazy_boolean/
diff -u --strip-trailing-cr -r expected_mir_dump.lazy_boolean/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.lazy_boolean/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/generics.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/generics.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.generics
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.generics/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "generics" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.generics
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.generics/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.generics/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.generics/
diff -u --strip-trailing-cr -r expected_mir_dump.generics/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.generics/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/if_else.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/if_else.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if_else
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if_else/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "if_else" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if_else
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if_else/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.if_else/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if_else/
diff -u --strip-trailing-cr -r expected_mir_dump.if_else/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.if_else/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/overflow.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/overflow.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.overflow
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.overflow/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "overflow" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.overflow
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.overflow/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.overflow/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.overflow/
diff -u --strip-trailing-cr -r expected_mir_dump.overflow/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.overflow/
# Compile the test program with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/partial_eq.rs \
  $( grep -q '^\/\/ require-rust-edition-2018' ../coverage/partial_eq.rs && echo "--edition=2018" ) \
  -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview" \
  -Ztrim-diagnostic-paths=no \
  -Zinstrument-coverage \
  -Zdump-mir=InstrumentCoverage \
  -Zdump-mir-spanview \
  -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.partial_eq
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.partial_eq/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "partial_eq" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.partial_eq
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.partial_eq/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.partial_eq/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.partial_eq/
diff -u --strip-trailing-cr -r expected_mir_dump.partial_eq/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.partial_eq/
diff -u --strip-trailing-cr -r expected_mir_dump.partial_eq/partial_eq.{impl#7}-fmt.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.partial_eq/partial_eq.{impl#7}-fmt.-------.InstrumentCoverage.0.html
--- expected_mir_dump.partial_eq/partial_eq.{impl#7}-fmt.-------.InstrumentCoverage.0.html 2021-02-10 06:22:34.171085478 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.partial_eq/partial_eq.{impl#7}-fmt.-------.InstrumentCoverage.0.html 2021-02-10 07:08:46.144860577 +0000
@@ -73,41 +73,38 @@
 4:17-4:22: @0[2]: _3 = &amp;((*_1).0: usize)
 4:17-4:22: @0[4]: _4 = &amp;((*_1).1: usize)
 4:17-4:22: @0[6]: _5 = &amp;((*_1).2: usize)
-4:17-4:22: @0[9]: _7 = &amp;mut (*_2)
-4:17-4:22: @0[12]: _9 = const &quot;Version&quot;
-4:17-4:22: @0[13]: _8 = &amp;(*_9)
-4:17-4:22: @0.Call: _6 = std::fmt::Formatter::debug_struct(move _7, move _8) -&gt; [return: bb1, unwind: bb6]
-4:17-4:22: @1[2]: FakeRead(ForLet, _6)
-4:17-4:22: @1[7]: _12 = &amp;mut _6
-4:17-4:22: @1[8]: _11 = &amp;mut (*_12)
-4:17-4:22: @1[11]: _14 = const &quot;major&quot;
-4:17-4:22: @1[12]: _13 = &amp;(*_14)
-4:17-4:22: @1[17]: _18 = &amp;(*_3)
-4:17-4:22: @1[18]: _17 = &amp;_18
-4:17-4:22: @1[19]: _16 = &amp;(*_17)
-4:17-4:22: @1[20]: _15 = move _16 as &amp;dyn std::fmt::Debug (Pointer(Unsize))
-4:17-4:22: @1.Call: _10 = std::fmt::DebugStruct::field(move _11, move _13, move _15) -&gt; [return: bb2, unwind: bb6]
-4:17-4:22: @2[11]: _21 = &amp;mut _6
-4:17-4:22: @2[12]: _20 = &amp;mut (*_21)
-4:17-4:22: @2[15]: _23 = const &quot;minor&quot;
-4:17-4:22: @2[16]: _22 = &amp;(*_23)
-4:17-4:22: @2[21]: _27 = &amp;(*_4)
-4:17-4:22: @2[22]: _26 = &amp;_27
-4:17-4:22: @2[23]: _25 = &amp;(*_26)
-4:17-4:22: @2[24]: _24 = move _25 as &amp;dyn std::fmt::Debug (Pointer(Unsize))
-4:17-4:22: @2.Call: _19 = std::fmt::DebugStruct::field(move _20, move _22, move _24) -&gt; [return: bb3, unwind: bb6]
-4:17-4:22: @3[11]: _30 = &amp;mut _6
-4:17-4:22: @3[12]: _29 = &amp;mut (*_30)
-4:17-4:22: @3[15]: _32 = const &quot;patch&quot;
-4:17-4:22: @3[16]: _31 = &amp;(*_32)
-4:17-4:22: @3[21]: _36 = &amp;(*_5)
-4:17-4:22: @3[22]: _35 = &amp;_36
-4:17-4:22: @3[23]: _34 = &amp;(*_35)
-4:17-4:22: @3[24]: _33 = move _34 as &amp;dyn std::fmt::Debug (Pointer(Unsize))
-4:17-4:22: @3.Call: _28 = std::fmt::DebugStruct::field(move _29, move _31, move _33) -&gt; [return: bb4, unwind: bb6]
-4:17-4:22: @4[10]: _38 = &amp;mut _6
-4:17-4:22: @4[11]: _37 = &amp;mut (*_38)
-4:17-4:22: @4.Call: _0 = std::fmt::DebugStruct::finish(move _37) -&gt; [return: bb5, unwind: bb6]
+4:17-4:22: @0[10]: _8 = &amp;mut (*_2)
+4:17-4:22: @0[13]: _10 = const &quot;Version&quot;
+4:17-4:22: @0[14]: _9 = &amp;(*_10)
+4:17-4:22: @0.Call: _7 = std::fmt::Formatter::debug_struct(move _8, move _9) -&gt; [return: bb1, unwind: bb6]
+4:17-4:22: @1[2]: _6 = &amp;mut _7
+4:17-4:22: @1[3]: FakeRead(ForLet, _6)
+4:17-4:22: @1[7]: _12 = &amp;mut (*_6)
+4:17-4:22: @1[10]: _14 = const &quot;major&quot;
+4:17-4:22: @1[11]: _13 = &amp;(*_14)
+4:17-4:22: @1[16]: _18 = &amp;(*_3)
+4:17-4:22: @1[17]: _17 = &amp;_18
+4:17-4:22: @1[18]: _16 = &amp;(*_17)
+4:17-4:22: @1[19]: _15 = move _16 as &amp;dyn std::fmt::Debug (Pointer(Unsize))
+4:17-4:22: @1.Call: _11 = std::fmt::DebugStruct::field(move _12, move _13, move _15) -&gt; [return: bb2, unwind: bb6]
+4:17-4:22: @2[9]: _20 = &amp;mut (*_6)
+4:17-4:22: @2[12]: _22 = const &quot;minor&quot;
+4:17-4:22: @2[13]: _21 = &amp;(*_22)
+4:17-4:22: @2[18]: _26 = &amp;(*_4)
+4:17-4:22: @2[19]: _25 = &amp;_26
+4:17-4:22: @2[20]: _24 = &amp;(*_25)
+4:17-4:22: @2[21]: _23 = move _24 as &amp;dyn std::fmt::Debug (Pointer(Unsize))
+4:17-4:22: @2.Call: _19 = std::fmt::DebugStruct::field(move _20, move _21, move _23) -&gt; [return: bb3, unwind: bb6]
+4:17-4:22: @3[9]: _28 = &amp;mut (*_6)
+4:17-4:22: @3[12]: _30 = const &quot;patch&quot;
+4:17-4:22: @3[13]: _29 = &amp;(*_30)
+4:17-4:22: @3[18]: _34 = &amp;(*_5)
+4:17-4:22: @3[19]: _33 = &amp;_34
+4:17-4:22: @3[20]: _32 = &amp;(*_33)
+4:17-4:22: @3[21]: _31 = move _32 as &amp;dyn std::fmt::Debug (Pointer(Unsize))
+4:17-4:22: @3.Call: _27 = std::fmt::DebugStruct::field(move _28, move _29, move _31) -&gt; [return: bb4, unwind: bb6]
+4:17-4:22: @4[8]: _35 = &amp;mut (*_6)
+4:17-4:22: @4.Call: _0 = std::fmt::DebugStruct::finish(move _35) -&gt; [return: bb5, unwind: bb6]
 4:22-4:22: @5.Return: return"><span class="annotation">@0,1,2,3,4,5⦊</span>Debug<span class="annotation">⦉@0,1,2,3,4,5</span></span></span></span></div>
 </body>
 </html>
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
warning: function is never used: `unused_private_function`
  --> ../coverage/lib/used_crate.rs:45:4
45 | fn unused_private_function() {
   |    ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

warning: function is never used: `unused_fn`
  --> ../coverage/dead_code.rs:15:4
   |
15 | fn unused_fn() {
   |
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted

make: *** [Makefile:78: partial_eq] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/coverage-spanview

test result: FAILED. 209 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out; finished in 29.62s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.1-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:44:22
