plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_errors/src/emitter.rs at line 1961:
 
     let mut lim = 10;
     for num_digits in 1..MAX_DIGITS {
-        if num < lim { return num_digits; }
+        if num < lim {
+            return num_digits;
+        }
         lim = lim.wrapping_mul(10);
     MAX_DIGITS
     MAX_DIGITS
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/tiny_list/tests.rs" "/checkout/compiler/rustc_data_structures/src/small_c_str/tests.rs" "/checkout/compiler/rustc_data_structures/src/stable_hasher/tests.rs" "/checkout/compiler/rustc_data_structures/src/stable_hasher.rs" "/checkout/compiler/rustc_errors/src/emitter.rs" "/checkout/compiler/rustc_data_structures/src/binary_search_util/tests.rs" "/checkout/compiler/rustc_data_structures/src/binary_search_util/mod.rs" "/checkout/compiler/rustc_data_structures/src/fingerprint.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:12
