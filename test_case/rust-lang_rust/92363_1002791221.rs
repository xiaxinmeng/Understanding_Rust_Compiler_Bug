plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Diff in /checkout/src/tools/compiletest/src/runtest.rs at line 3522:
         }
 
         lazy_static! {
-            static ref V0_BACK_REF_PREFIX_RE: Regex =
-                Regex::new(r"\(_R.*?B[0-9a-zA-Z]_").unwrap();
+            static ref V0_BACK_REF_PREFIX_RE: Regex = Regex::new(r"\(_R.*?B[0-9a-zA-Z]_").unwrap();
             static ref V0_BACK_REF_RE: Regex = Regex::new(r"B[0-9a-zA-Z]_").unwrap();
 
Diff in /checkout/src/tools/compiletest/src/runtest.rs at line 3530:
Diff in /checkout/src/tools/compiletest/src/runtest.rs at line 3530:
         const V0_BACK_REF_PLACEHOLDER: &str = r"B<REF>_";
         if V0_BACK_REF_PREFIX_RE.is_match(&normalized) {
             // Normalize back references (see RFC 2603)
-            normalized = V0_BACK_REF_RE.replace_all(&normalized, V0_BACK_REF_PLACEHOLDER).into_owned();
+            normalized =
+                V0_BACK_REF_RE.replace_all(&normalized, V0_BACK_REF_PLACEHOLDER).into_owned();
 
 
         // Custom normalization rules
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/html-checker/main.rs" "/checkout/src/tools/compiletest/src/raise_fd_limit.rs" "/checkout/src/tools/compiletest/src/errors.rs" "/checkout/src/tools/compiletest/src/compute_diff.rs" "/checkout/src/tools/compiletest/src/common.rs" "/checkout/src/tools/unstable-book-gen/src/main.rs" "/checkout/src/tools/compiletest/src/runtest.rs" "/checkout/src/librustdoc/docfs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
